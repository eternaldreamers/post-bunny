use gpui::{AppContext, ClickEvent, Context, Window};

use crate::config::{dev_values, DEMO_SOL_PASSWORD};
use crate::domain::customer::{AccountStatus, Customer, CustomerDraft, LoginType};
use crate::domain::navigation::{DetailTab, Page};
use crate::i18n;
use crate::storage;
use crate::ui::text_input::TextInput;

use super::{CustomerForm, Panel, ToastKind};

impl CustomerForm {
    pub(super) fn new(language: crate::i18n::Language, cx: &mut Context<Panel>) -> Self {
        let password = cx.new(|cx| TextInput::new(i18n::password_placeholder(language), cx));
        password.update(cx, |input, cx| input.set_masked(true, cx));
        Self {
            open: false,
            editing: None,
            login_type: LoginType::Dni,
            dni: cx.new(|cx| TextInput::new("", cx)),
            ruc: cx.new(|cx| TextInput::new("", cx)),
            sol_user: cx.new(|cx| TextInput::new("", cx)),
            password,
            show_password: false,
            error: None,
        }
    }

    pub(super) fn reset(&mut self, cx: &mut Context<Panel>) {
        self.editing = None;
        self.login_type = LoginType::Dni;
        self.show_password = false;
        self.error = None;
        self.dni.update(cx, |input, cx| input.clear(cx));
        self.ruc.update(cx, |input, cx| input.clear(cx));
        self.sol_user.update(cx, |input, cx| input.clear(cx));
        self.password.update(cx, |input, cx| {
            input.clear(cx);
            input.set_masked(true, cx);
        });
    }

    pub(super) fn apply_draft(
        &mut self,
        draft: &CustomerDraft,
        editing: Option<u64>,
        cx: &mut Context<Panel>,
    ) {
        self.editing = editing;
        self.login_type = draft.login_type;
        self.show_password = false;
        self.error = None;
        self.dni
            .update(cx, |input, cx| input.set_text(draft.dni.clone(), cx));
        self.ruc
            .update(cx, |input, cx| input.set_text(draft.ruc.clone(), cx));
        self.sol_user
            .update(cx, |input, cx| input.set_text(draft.sol_user.clone(), cx));
        self.password.update(cx, |input, cx| {
            input.set_text(draft.password.clone(), cx);
            input.set_masked(true, cx);
        });
    }

    pub(super) fn set_demo_dni(&mut self, cx: &mut Context<Panel>) {
        self.dni
            .update(cx, |input, cx| input.set_text(dev_values::DEMO_DNI, cx));
        self.password.update(cx, |input, cx| {
            input.set_text(DEMO_SOL_PASSWORD, cx);
            input.set_masked(true, cx);
        });
    }

    pub(super) fn set_demo_ruc(&mut self, cx: &mut Context<Panel>) {
        self.ruc
            .update(cx, |input, cx| input.set_text(dev_values::DEMO_RUC, cx));
        self.sol_user.update(cx, |input, cx| {
            input.set_text(dev_values::DEMO_SOL_USER, cx)
        });
        self.password.update(cx, |input, cx| {
            input.set_text(DEMO_SOL_PASSWORD, cx);
            input.set_masked(true, cx);
        });
    }

    fn focus_handle_for(&self, login_type: LoginType, cx: &gpui::App) -> gpui::FocusHandle {
        match login_type {
            LoginType::Dni => self.dni.read(cx).focus_handle.clone(),
            LoginType::Ruc => self.ruc.read(cx).focus_handle.clone(),
        }
    }
}

impl Panel {
    pub(super) fn begin_new_customer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.form.reset(cx);
        self.form.open = true;
        let handle = self.form.focus_handle_for(LoginType::Dni, cx);
        window.focus(&handle);
        cx.notify();
    }

    pub fn open_new_customer(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.begin_new_customer(window, cx);
    }

    pub fn open_edit_customer(&mut self, id: u64, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .customer(id)
            .is_some_and(|customer| customer.password.is_empty())
        {
            if let Some(password) = storage::load_password(id) {
                if let Some(customer) = self.customer_mut(id) {
                    customer.password = password;
                }
            }
        }
        let Some(customer) = self.customer(id) else {
            return;
        };
        let draft = CustomerDraft::from_customer(customer);
        let login_type = draft.login_type;
        self.form.apply_draft(&draft, Some(id), cx);
        self.form.open = true;
        let handle = self.form.focus_handle_for(login_type, cx);
        window.focus(&handle);
        cx.notify();
    }

    pub fn close_customer_form(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.form.open = false;
        self.form.error = None;
        window.focus(&self.root_focus);
        cx.notify();
    }

    pub fn set_login_type(
        &mut self,
        login_type: LoginType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.form.login_type == login_type {
            return;
        }
        self.form.login_type = login_type;
        self.form.error = None;
        let handle = self.form.focus_handle_for(login_type, cx);
        window.focus(&handle);
        cx.notify();
    }

    pub fn toggle_form_password(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.form.show_password = !self.form.show_password;
        let masked = !self.form.show_password;
        self.form
            .password
            .update(cx, |input, cx| input.set_masked(masked, cx));
        cx.notify();
    }

    pub fn toggle_password_visibility(
        &mut self,
        _: &ClickEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.password_visible = !self.password_visible;
        cx.notify();
    }

    pub fn save_customer(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.submit_customer(window, cx);
    }

    pub fn submit_customer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let draft = self.read_draft(cx);
        if let Err(error) = draft.validate(self.language) {
            self.form.error = Some(error);
            cx.notify();
            return;
        }

        let password = draft.password.clone();
        let (saved_id, success_message) = match self.form.editing {
            Some(id) => (id, self.update_customer(id, &draft)),
            None => (
                self.create_customer(&draft),
                i18n::t(self.language, "Customer added.", "Cliente agregado."),
            ),
        };

        let keyring_error = storage::save_password(saved_id, &password).err();
        self.persist_customers();
        self.form.open = false;
        self.form.error = None;
        window.focus(&self.root_focus);

        if let Some(error) = keyring_error {
            self.show_toast(
                format!(
                    "{} {error}",
                    i18n::t(
                        self.language,
                        "Saved locally, but the keystore rejected the password:",
                        "Guardado localmente, pero el keystore rechazó la contraseña:",
                    )
                ),
                ToastKind::Danger,
                cx,
            );
        } else {
            self.show_toast(success_message, ToastKind::Success, cx);
        }
        cx.notify();
    }

    fn read_draft(&self, cx: &gpui::App) -> CustomerDraft {
        CustomerDraft {
            login_type: self.form.login_type,
            dni: self.form.dni.read(cx).text().trim().to_string(),
            ruc: self.form.ruc.read(cx).text().trim().to_string(),
            sol_user: self.form.sol_user.read(cx).text().trim().to_string(),
            password: self.form.password.read(cx).text(),
        }
    }

    fn update_customer(&mut self, id: u64, draft: &CustomerDraft) -> &'static str {
        let ruc = draft.resolved_ruc().unwrap_or_default();
        let (dni, sol_user) = match draft.login_type {
            LoginType::Dni => (Some(draft.dni.clone()), None),
            LoginType::Ruc => (None, Some(draft.sol_user.clone())),
        };

        let mut identity_changed = false;
        if let Some(customer) = self.customer_mut(id) {
            identity_changed = customer.login_type != draft.login_type
                || customer.ruc != ruc
                || customer.sol_user != sol_user;
            customer.login_type = draft.login_type;
            customer.dni = dni;
            customer.ruc = ruc;
            customer.sol_user = sol_user;
            customer.password = draft.password.clone();
            if identity_changed {
                customer.display_name = None;
                customer.status = AccountStatus::Pending;
                customer.last_sync = None;
                customer.messages.clear();
            } else if customer.status == AccountStatus::SessionExpired {
                customer.status = AccountStatus::Pending;
            }
        }
        if identity_changed {
            self.sessions.remove(&id);
            storage::delete_session(id);
        }

        i18n::t(
            self.language,
            "Credentials updated.",
            "Credenciales actualizadas.",
        )
    }

    fn create_customer(&mut self, draft: &CustomerDraft) -> u64 {
        let ruc = draft.resolved_ruc().unwrap_or_default();
        let (dni, sol_user) = match draft.login_type {
            LoginType::Dni => (Some(draft.dni.clone()), None),
            LoginType::Ruc => (None, Some(draft.sol_user.clone())),
        };
        let id = self.next_customer_id;
        self.next_customer_id += 1;
        self.customers.push(Customer {
            id,
            display_name: None,
            login_type: draft.login_type,
            dni,
            ruc,
            sol_user,
            password: draft.password.clone(),
            status: AccountStatus::Pending,
            last_sync: None,
            notes: i18n::t(self.language, "New SUNAT customer.", "Nuevo cliente SUNAT.").into(),
            messages: Vec::new(),
        });
        self.selected_customer_id = Some(id);
        self.detail_tab = DetailTab::Overview;
        self.page = Page::Customers;
        id
    }

    pub fn ask_delete_customer(&mut self, id: u64, cx: &mut Context<Self>) {
        self.delete_customer = Some(id);
        cx.notify();
    }

    pub fn cancel_delete_customer(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.delete_customer = None;
        window.focus(&self.root_focus);
        cx.notify();
    }

    pub fn confirm_delete_customer(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(id) = self.delete_customer.take() {
            let name = self
                .customer(id)
                .map(Customer::title)
                .unwrap_or_else(|| i18n::t(self.language, "Customer", "Cliente").into());
            self.customers.retain(|customer| customer.id != id);
            self.alerts.retain(|alert| alert.customer_id != id);
            self.sessions.remove(&id);
            storage::delete_password(id);
            storage::delete_session(id);
            if self.selected_customer_id == Some(id) {
                self.selected_customer_id = None;
                self.reading_message_id = None;
            }
            self.persist_customers();
            self.persist_alerts();
            self.show_toast(
                i18n::deleted_customer(self.language, &name),
                ToastKind::Danger,
                cx,
            );
        }
        window.focus(&self.root_focus);
        cx.notify();
    }
}
