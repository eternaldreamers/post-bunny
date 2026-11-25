mod alerts;
mod bootstrap;
mod dev;
mod forms;
mod keyboard;
mod language;
mod mailbox;
mod navigation;
mod persistence;
mod preview;
mod queries;
mod render;
mod sync;
mod toast;
mod types;

use std::collections::{HashMap, HashSet};

use gpui::{Context, Entity, FocusHandle};

use crate::domain::alert::{Alert, AlertFilter};
use crate::domain::customer::Customer;
use crate::domain::navigation::{DetailTab, Page};
use crate::i18n::Language;
use crate::sunat::Session;
use crate::ui::text_input::TextInput;

pub(crate) use types::{
    AttachmentPreview, AttachmentPreviewState, CustomerForm, SyncJob, Toast, ToastKind,
};

pub struct Panel {
    pub(crate) page: Page,
    pub(crate) customers: Vec<Customer>,
    pub(crate) alerts: Vec<Alert>,
    pub(crate) next_customer_id: u64,
    pub(crate) next_toast_id: u64,
    pub(crate) drawer_open: bool,
    pub(crate) customer_search: Entity<TextInput>,
    pub(crate) selected_customer_id: Option<u64>,
    pub(crate) detail_tab: DetailTab,
    pub(crate) reading_message_id: Option<u64>,
    pub(crate) alert_filter: AlertFilter,
    pub(crate) password_visible: bool,
    pub(crate) language: Language,
    pub(crate) language_menu_open: bool,
    pub(crate) form: CustomerForm,
    pub(crate) delete_customer: Option<u64>,
    pub(crate) sessions: HashMap<u64, Session>,
    pub(crate) syncing: bool,
    pub(crate) loading_messages: HashSet<u64>,
    pub(crate) next_alert_id: u64,
    pub(crate) attachment_preview: Option<AttachmentPreview>,
    pub(crate) toast: Option<Toast>,
    pub(crate) root_focus: FocusHandle,
    pub(crate) today: String,
}

impl Panel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        bootstrap::build(cx)
    }
}
