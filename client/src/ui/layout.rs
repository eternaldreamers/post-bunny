#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SidebarMode {
    Expanded,
    Rail,
    Hidden,
}

pub const SIDEBAR_EXPANDED_WIDTH: f32 = 248.0;
pub const SIDEBAR_RAIL_WIDTH: f32 = 76.0;
pub const SIDEBAR_DRAWER_WIDTH: f32 = 252.0;
pub const CONTENT_MAX_WIDTH: f32 = 1440.0;
pub const CONTENT_PADDING: f32 = 28.0;
pub const CONTENT_PADDING_COMPACT: f32 = 16.0;
pub const TOPBAR_HEIGHT: f32 = 60.0;
pub const STATUSBAR_HEIGHT: f32 = 28.0;

const SIDEBAR_EXPANDED_AT: f32 = 1180.0;
const SIDEBAR_RAIL_AT: f32 = 900.0;
const COMPACT_AT: f32 = 700.0;
const DETAIL_COMPACT_AT: f32 = 720.0;
const FORM_WIDE_AT: f32 = 540.0;
const INBOX_WIDE_AT: f32 = 1080.0;
const CUSTOMER_LOGIN_AT: f32 = 1080.0;
const CUSTOMER_MESSAGES_AT: f32 = 920.0;
const CUSTOMER_ALERTS_AT: f32 = 800.0;
const CUSTOMER_LAST_SYNC_AT: f32 = 1180.0;
const CUSTOMER_HEADER_AT: f32 = 780.0;

pub fn sidebar_mode(width: f32) -> SidebarMode {
    if width >= SIDEBAR_EXPANDED_AT {
        SidebarMode::Expanded
    } else if width >= SIDEBAR_RAIL_AT {
        SidebarMode::Rail
    } else {
        SidebarMode::Hidden
    }
}

pub fn is_compact(width: f32) -> bool {
    width < COMPACT_AT
}

pub fn is_detail_compact(width: f32) -> bool {
    width < DETAIL_COMPACT_AT
}

pub fn is_form_wide(width: f32) -> bool {
    width > FORM_WIDE_AT
}

pub fn is_inbox_wide(width: f32) -> bool {
    width >= INBOX_WIDE_AT
}

pub fn show_customer_login(width: f32) -> bool {
    width >= CUSTOMER_LOGIN_AT
}

pub fn show_customer_messages(width: f32) -> bool {
    width >= CUSTOMER_MESSAGES_AT
}

pub fn show_customer_alerts(width: f32) -> bool {
    width >= CUSTOMER_ALERTS_AT
}

pub fn show_customer_last_sync(width: f32) -> bool {
    width >= CUSTOMER_LAST_SYNC_AT
}

pub fn show_customer_header(width: f32) -> bool {
    width >= CUSTOMER_HEADER_AT
}

pub fn content_padding(compact: bool) -> f32 {
    if compact {
        CONTENT_PADDING_COMPACT
    } else {
        CONTENT_PADDING
    }
}

pub fn shortcut_modifier() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌘"
    } else {
        "Ctrl+"
    }
}
