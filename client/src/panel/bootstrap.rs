use std::collections::{HashMap, HashSet};

use gpui::{AppContext, Context};

use crate::domain::alert::{Alert, AlertFilter};
use crate::domain::customer::Customer;
use crate::domain::date::today_label;
use crate::domain::navigation::{DetailTab, Page};
use crate::domain::seed::{seed_alerts, seed_customers};
use crate::i18n;
use crate::storage;
use crate::sunat::Session;
use crate::ui::text_input::TextInput;

use super::{CustomerForm, Panel};

pub(super) fn build(cx: &mut Context<Panel>) -> Panel {
    let customers = load_customers();
    let alerts = load_alerts();
    let next_customer_id = customers
        .iter()
        .map(|customer| customer.id)
        .max()
        .unwrap_or(0)
        + 1;
    let next_alert_id = alerts.iter().map(|alert| alert.id).max().unwrap_or(0) + 1;
    let language = storage::load_language();
    let customer_search =
        cx.new(|cx| TextInput::new(i18n::search_placeholder(language), cx).with_icon("search"));
    let form = CustomerForm::new(language, cx);

    let mut panel = Panel {
        page: Page::Home,
        customers,
        alerts,
        next_customer_id,
        next_toast_id: 0,
        drawer_open: false,
        customer_search,
        selected_customer_id: None,
        detail_tab: DetailTab::Overview,
        reading_message_id: None,
        alert_filter: AlertFilter::All,
        password_visible: false,
        language,
        language_menu_open: false,
        form,
        delete_customer: None,
        sessions: HashMap::new(),
        syncing: false,
        loading_messages: HashSet::new(),
        next_alert_id,
        attachment_preview: None,
        toast: None,
        root_focus: cx.focus_handle(),
        today: today_label(language),
    };

    load_sessions(&mut panel);
    super::dev::apply(&mut panel, cx);
    cx.observe(&panel.customer_search, |_, _, cx| cx.notify())
        .detach();
    panel
}

fn load_customers() -> Vec<Customer> {
    if storage::customers_file_exists() {
        return storage::load_customers();
    }
    let customers = seed_customers();
    let _ = storage::save_customers(&customers);
    customers
}

fn load_alerts() -> Vec<Alert> {
    if storage::alerts_file_exists() {
        return storage::load_alerts();
    }
    let alerts = seed_alerts();
    let _ = storage::save_alerts(&alerts);
    alerts
}

fn load_sessions(panel: &mut Panel) {
    for customer in &panel.customers {
        if let Some(record) = storage::load_session(customer.id) {
            if let Ok(session) =
                Session::from_record(record, storage::session_cookie_path(customer.id))
            {
                panel.sessions.insert(customer.id, session);
            }
        }
    }
}
