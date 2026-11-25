use std::env;

use gpui::Context;

use crate::config::{dev_values, env_names};
use crate::domain::customer::{CustomerDraft, LoginType};
use crate::domain::navigation::{DetailTab, Page};

use super::Panel;

pub(super) fn apply(panel: &mut Panel, cx: &mut Context<Panel>) {
    if let Ok(page) = env::var(env_names::PAGE) {
        match page.as_str() {
            "customers" => panel.page = Page::Customers,
            "alerts" => panel.page = Page::Alerts,
            _ => {}
        }
    }

    if let Ok(id) = env::var(env_names::CUSTOMER) {
        if let Ok(id) = id.parse::<u64>() {
            panel.selected_customer_id = Some(id);
            panel.page = Page::Customers;
        }
    }

    if let Ok(tab) = env::var(env_names::TAB) {
        panel.detail_tab = match tab.as_str() {
            "credentials" => DetailTab::Credentials,
            "inbox" => DetailTab::Inbox,
            _ => DetailTab::Overview,
        };
    }

    if let Ok(message) = env::var(env_names::MESSAGE) {
        if let Ok(message) = message.parse::<u64>() {
            panel.reading_message_id = Some(message);
        }
    }

    if let Ok(search) = env::var(env_names::SEARCH) {
        panel
            .customer_search
            .update(cx, |input, cx| input.set_text(search, cx));
    }

    if env::var(env_names::DRAWER).is_ok() {
        panel.drawer_open = true;
    }

    apply_modal(panel, cx);

    if let Ok(login) = env::var(env_names::LOGIN) {
        if login == dev_values::LOGIN_RUC {
            panel.form.login_type = LoginType::Ruc;
            if panel.form.open && panel.form.editing.is_none() {
                panel.form.set_demo_ruc(cx);
            }
        }
    }

    if env::var(env_names::DELETE).is_ok() {
        panel.delete_customer = panel.selected_customer_id.or(Some(1));
    }

    if let Ok(id) = env::var(env_names::ALERT) {
        if let Ok(id) = id.parse::<u64>() {
            panel.open_alert(id, cx);
        }
    }
}

fn apply_modal(panel: &mut Panel, cx: &mut Context<Panel>) {
    let Ok(modal) = env::var(env_names::MODAL) else {
        return;
    };
    if modal != dev_values::MODAL_NEW && modal != dev_values::MODAL_EDIT {
        return;
    }

    panel.form.open = true;
    panel.form.reset(cx);

    if modal == dev_values::MODAL_EDIT {
        let editing = panel.selected_customer_id;
        let draft = editing
            .and_then(|id| panel.customer(id))
            .map(CustomerDraft::from_customer);
        if let Some(draft) = draft {
            panel.form.apply_draft(&draft, editing, cx);
        }
    } else {
        panel.form.set_demo_dni(cx);
    }
}
