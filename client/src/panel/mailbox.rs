use gpui::{ClickEvent, Context, Window};

use crate::domain::navigation::{DetailTab, Page};
use crate::i18n;
use crate::sunat;

use super::{Panel, ToastKind};

impl Panel {
    pub fn open_message(&mut self, cod_mensaje: u64, cx: &mut Context<Self>) {
        self.reading_message_id = Some(cod_mensaje);
        if let Some(customer_id) = self.selected_customer_id {
            if let Some(customer) = self.customer_mut(customer_id) {
                if let Some(message) = customer.message_mut(cod_mensaje) {
                    message.read = true;
                }
            }
            self.resolve_alert_for_message(customer_id, cod_mensaje);
            self.load_message_body(customer_id, cod_mensaje, cx);
        }
        self.persist_customers();
        cx.notify();
    }

    pub fn close_message(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.reading_message_id = None;
        cx.notify();
    }

    pub fn mark_message_unread(&mut self, cod_mensaje: u64, cx: &mut Context<Self>) {
        if let Some(customer_id) = self.selected_customer_id {
            if let Some(customer) = self.customer_mut(customer_id) {
                if let Some(message) = customer.message_mut(cod_mensaje) {
                    message.read = false;
                }
            }
        }
        self.reading_message_id = None;
        self.persist_customers();
        self.show_toast(
            i18n::t(
                self.language,
                "Message marked as unread.",
                "Mensaje marcado como no leído.",
            ),
            ToastKind::Info,
            cx,
        );
    }

    pub fn mark_all_messages_read(&mut self, customer_id: u64, cx: &mut Context<Self>) {
        if let Some(customer) = self.customer_mut(customer_id) {
            for message in &mut customer.messages {
                message.read = true;
            }
        }
        self.persist_customers();
        self.show_toast(
            i18n::t(
                self.language,
                "Mailbox marked as read.",
                "Bandeja marcada como leída.",
            ),
            ToastKind::Success,
            cx,
        );
    }

    pub fn jump_to_message(&mut self, customer_id: u64, cod_mensaje: u64, cx: &mut Context<Self>) {
        self.page = Page::Customers;
        self.selected_customer_id = Some(customer_id);
        self.detail_tab = DetailTab::Inbox;
        self.reading_message_id = Some(cod_mensaje);
        self.password_visible = false;
        if let Some(customer) = self.customer_mut(customer_id) {
            if let Some(message) = customer.message_mut(cod_mensaje) {
                message.read = true;
            }
        }
        self.resolve_alert_for_message(customer_id, cod_mensaje);
        self.load_message_body(customer_id, cod_mensaje, cx);
        self.persist_customers();
        cx.notify();
    }

    pub fn load_message_body(
        &mut self,
        customer_id: u64,
        cod_mensaje: u64,
        cx: &mut Context<Self>,
    ) {
        let Some(message) = self
            .customer(customer_id)
            .and_then(|customer| customer.message(cod_mensaje))
        else {
            return;
        };
        if message.body_loaded || self.loading_messages.contains(&cod_mensaje) {
            return;
        }
        let context_tipo = message.context_tipo.clone();
        let Some(session) = self.sessions.get(&customer_id).cloned() else {
            self.show_toast(
                i18n::t(
                    self.language,
                    "Sync this customer to load message bodies.",
                    "Sincroniza este cliente para cargar el cuerpo de los mensajes.",
                ),
                ToastKind::Info,
                cx,
            );
            return;
        };

        self.loading_messages.insert(cod_mensaje);
        cx.notify();

        let executor = cx.background_executor().clone();
        let task = executor.spawn(async move {
            sunat::fetch_message_detail(&session, cod_mensaje, &context_tipo)
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |panel, cx| {
                panel.loading_messages.remove(&cod_mensaje);
                match result {
                    Ok(detail) => {
                        if let Some(customer) = panel.customer_mut(customer_id) {
                            if let Some(message) = customer.message_mut(cod_mensaje) {
                                message.body = detail.body;
                                message.body_loaded = true;
                                message.attachment_count = detail.attachments.len();
                                message.attachments = detail.attachments;
                                message.read = true;
                            }
                        }
                        panel.resolve_alert_for_message(customer_id, cod_mensaje);
                        panel.persist_customers();
                        cx.notify();
                    }
                    Err(error) => {
                        panel.show_toast(
                            format!(
                                "{} {error}",
                                i18n::t(
                                    panel.language,
                                    "Could not load message:",
                                    "No se pudo cargar el mensaje:",
                                )
                            ),
                            ToastKind::Danger,
                            cx,
                        );
                    }
                }
            })
            .ok();
        })
        .detach();
    }
}
