use std::env;
use std::time::Duration;

pub const APP_NAME: &str = "Post Bunny";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FONT_FAMILY: &str = ".SystemUIFont";
pub const BASE_FONT_SIZE: f32 = 14.0;

pub const DEFAULT_WINDOW_WIDTH: f32 = 1320.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 900.0;
pub const MIN_WINDOW_WIDTH: f32 = 560.0;
pub const MIN_WINDOW_HEIGHT: f32 = 520.0;

pub const TOAST_DURATION: Duration = Duration::from_secs(4);
pub const DEMO_SOL_PASSWORD: &str = "demo-sol-pass";
pub const ALERT_SOURCE_MAILBOX: &str = "Buzón";
pub const DEFAULT_LABEL_COLOR: u32 = 0x74ade8;

pub mod env_names {
    pub const WINDOW_WIDTH: &str = "PANEL_WIDTH";
    pub const WINDOW_HEIGHT: &str = "PANEL_HEIGHT";
    pub const PAGE: &str = "PANEL_PAGE";
    pub const CUSTOMER: &str = "PANEL_CUSTOMER";
    pub const TAB: &str = "PANEL_TAB";
    pub const MESSAGE: &str = "PANEL_MESSAGE";
    pub const SEARCH: &str = "PANEL_SEARCH";
    pub const DRAWER: &str = "PANEL_DRAWER";
    pub const MODAL: &str = "PANEL_MODAL";
    pub const LOGIN: &str = "PANEL_LOGIN";
    pub const DELETE: &str = "PANEL_DELETE";
    pub const ALERT: &str = "PANEL_ALERT";
}

pub mod dev_values {
    pub const MODAL_NEW: &str = "new";
    pub const MODAL_EDIT: &str = "edit";
    pub const LOGIN_RUC: &str = "ruc";
    pub const DEMO_DNI: &str = "48703474";
    pub const DEMO_RUC: &str = "20123456786";
    pub const DEMO_SOL_USER: &str = "ANDESOL";
}

pub fn initial_window_size() -> (f32, f32) {
    (
        env_number(env_names::WINDOW_WIDTH, DEFAULT_WINDOW_WIDTH),
        env_number(env_names::WINDOW_HEIGHT, DEFAULT_WINDOW_HEIGHT),
    )
}

fn env_number(name: &str, fallback: f32) -> f32 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(fallback)
}
