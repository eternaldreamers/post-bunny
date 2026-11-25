use std::collections::HashMap;

use gpui::Context;

use crate::config::ALERT_SOURCE_MAILBOX;
use crate::domain::alert::{Alert, AlertState};
use crate::domain::customer::{AccountStatus, Customer, LoginType};
use crate::domain::message::SunatMessage;
use crate::i18n;
use crate::storage;
use crate::sunat::{self, SunatCredentials, SyncOutcome};

use super::{Panel, SyncJob, ToastKind};

impl Panel {
    pub fn sync_all_now(&mut self, cx: &mut Context<Self>) {
        if self.syncing {
            return;
        }
        self.load_credentials();
        let mut jobs = self
            .customers
            .iter()
            .filter_map(|customer| self.sync_job(customer))
            .collect::<Vec<_>>();
        jobs.sort_by_key(|job| {
            !self
                .customer(job.customer_id)
                .is_some_and(|customer| customer.status == AccountStatus::SessionExpired)
        });
        if jobs.is_empty() {
            self.show_toast(
                i18n::t(
                    self.language,
                    "Add SOL credentials to a customer before syncing.",
                    "Agrega credenciales SOL a un cliente antes de sincronizar.",
                ),
                ToastKind::Info,
                cx,
            );
            return;
        }
        self.syncing = true;
        cx.notify();
        self.start_sync(jobs, cx);
    }

    pub fn sync_one(&mut self, customer_id: u64, cx: &mut Context<Self>) {
        if self.syncing {
            return;
        }
        if self
            .customer(customer_id)
            .is_some_and(|customer| customer.password.is_empty())
        {
            if let Some(password) = storage::load_password(customer_id) {
                if let Some(customer) = self.customer_mut(customer_id) {
                    customer.password = password;
                }
            }
        }
        let Some(customer) = self.customer(customer_id) else {
            return;
        };
        let Some(job) = self.sync_job(customer) else {
            self.show_toast(
                i18n::t(
                    self.language,
                    "Edit the credentials and enter the real SOL password before syncing.",
                    "Edita las credenciales e ingresa la contraseña SOL real antes de sincronizar.",
                ),
                ToastKind::Info,
                cx,
            );
            return;
        };
        self.syncing = true;
        cx.notify();
        self.start_sync(vec![job], cx);
    }

    fn credentials_for(customer: &Customer) -> Option<SunatCredentials> {
        if !customer.has_real_password() {
            return None;
        }
        match customer.login_type {
            LoginType::Dni => Some(SunatCredentials {
                login_type: LoginType::Dni,
                dni: Some(customer.dni.clone()?),
                ruc: None,
                sol_user: None,
                password: customer.password.clone(),
            }),
            LoginType::Ruc => Some(SunatCredentials {
                login_type: LoginType::Ruc,
                dni: None,
                ruc: Some(customer.ruc.clone()),
                sol_user: Some(customer.sol_user.clone()?),
                password: customer.password.clone(),
            }),
        }
    }

    fn load_credentials(&mut self) {
        for customer in &mut self.customers {
            if customer.password.is_empty() {
                if let Some(password) = storage::load_password(customer.id) {
                    customer.password = password;
                }
            }
        }
    }

    fn sync_job(&self, customer: &Customer) -> Option<SyncJob> {
        let credentials = Self::credentials_for(customer);
        let existing = self.sessions.get(&customer.id).cloned();
        if credentials.is_none() && existing.is_none() {
            return None;
        }
        Some(SyncJob {
            customer_id: customer.id,
            credentials,
            existing,
            jar_path: storage::session_cookie_path(customer.id),
        })
    }

    fn start_sync(&mut self, jobs: Vec<SyncJob>, cx: &mut Context<Self>) {
        let executor = cx.background_executor().clone();
        let task = executor.spawn(async move {
            jobs.into_iter()
                .map(|job| {
                    sunat::sync_customer(
                        job.customer_id,
                        job.credentials.as_ref(),
                        job.existing,
                        job.jar_path,
                    )
                })
                .collect::<Vec<_>>()
        });
        cx.spawn(async move |this, cx| {
            let results = task.await;
            this.update(cx, |panel, cx| {
                panel.syncing = false;
                panel.apply_sync_results(results, cx);
            })
            .ok();
        })
        .detach();
    }

    fn apply_sync_results(&mut self, results: Vec<SyncOutcome>, cx: &mut Context<Self>) {
        let mut ok_count = 0;
        let mut error_count = 0;
        let mut total_messages = 0;
        let mut new_alerts = 0;
        let mut errors = Vec::new();

        for outcome in results {
            let SyncOutcome {
                customer_id,
                ruc,
                display_name,
                messages,
                last_sync,
                session,
                error,
            } = outcome;
            let customer_name = self
                .customer(customer_id)
                .map(Customer::title)
                .unwrap_or_else(|| i18n::t(self.language, "Customer", "Cliente").into());

            if let Some(error) = error {
                error_count += 1;
                if let Some(customer) = self.customer_mut(customer_id) {
                    customer.status = AccountStatus::SessionExpired;
                }
                self.sessions.remove(&customer_id);
                storage::delete_session(customer_id);
                eprintln!("SUNAT sync failed for {customer_name}: {error}");
                errors.push(format!("{customer_name}: {error}"));
                continue;
            }

            ok_count += 1;
            total_messages += messages.len();
            let merged = self.merge_messages(customer_id, &messages);
            if let Some(customer) = self.customer_mut(customer_id) {
                if let Some(name) = display_name {
                    customer.display_name = Some(name);
                }
                if !ruc.is_empty() {
                    customer.ruc = ruc;
                }
                customer.status = AccountStatus::Connected;
                customer.last_sync = Some(last_sync);
                customer.messages = merged;
            }
            if let Some(session) = session {
                if let Err(error) = storage::save_session(customer_id, &session.record()) {
                    eprintln!("Could not persist SUNAT session: {error}");
                }
                self.sessions.insert(customer_id, session);
            }
            new_alerts += self.sync_alerts_for_customer(customer_id);
        }

        self.persist_customers();
        self.persist_alerts();

        let first_error = errors.first().cloned().unwrap_or_default();
        let (summary, kind) = if error_count == 0 {
            (
                i18n::sync_summary(self.language, ok_count, total_messages, new_alerts),
                ToastKind::Success,
            )
        } else if ok_count > 0 {
            (
                i18n::sync_partial(self.language, ok_count, error_count, &first_error),
                ToastKind::Danger,
            )
        } else {
            (
                i18n::sync_failed(self.language, &first_error),
                ToastKind::Danger,
            )
        };
        self.show_toast(summary, kind, cx);
    }

    fn merge_messages(&self, customer_id: u64, synced: &[SunatMessage]) -> Vec<SunatMessage> {
        let previous: HashMap<u64, SunatMessage> = self
            .customer(customer_id)
            .map(|customer| {
                customer
                    .messages
                    .iter()
                    .cloned()
                    .map(|message| (message.cod_mensaje, message))
                    .collect()
            })
            .unwrap_or_default();

        synced
            .iter()
            .map(|message| {
                let Some(old) = previous.get(&message.cod_mensaje) else {
                    return message.clone();
                };
                if !old.body_loaded {
                    return message.clone();
                }
                let mut merged = message.clone();
                merged.body = old.body.clone();
                merged.body_loaded = true;
                merged.attachments = old.attachments.clone();
                merged.attachment_count = merged.attachment_count.max(old.attachment_count);
                merged
            })
            .collect()
    }

    fn sync_alerts_for_customer(&mut self, customer_id: u64) -> usize {
        let Some(customer) = self.customer(customer_id).cloned() else {
            return 0;
        };
        let messages = customer.messages;
        let mut created = 0;

        for message in messages.iter().filter(|message| !message.read) {
            let exists = self.alerts.iter().any(|alert| {
                alert.customer_id == customer_id && alert.cod_mensaje == Some(message.cod_mensaje)
            });
            if exists {
                continue;
            }
            self.alerts.push(Alert::open(
                self.next_alert_id,
                customer_id,
                message,
                ALERT_SOURCE_MAILBOX,
            ));
            self.next_alert_id += 1;
            created += 1;
        }

        for alert in &mut self.alerts {
            if alert.customer_id != customer_id {
                continue;
            }
            let Some(cod_mensaje) = alert.cod_mensaje else {
                continue;
            };
            if let Some(message) = messages
                .iter()
                .find(|message| message.cod_mensaje == cod_mensaje)
            {
                if message.read && alert.state == AlertState::Open {
                    alert.state = AlertState::Resolved;
                }
            }
        }

        created
    }
}
