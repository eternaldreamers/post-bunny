use std::path::PathBuf;

const APP_DIR: &str = "PostBunny";
const CUSTOMERS_FILE: &str = "customers.json";
const ALERTS_FILE: &str = "alerts.json";
const SETTINGS_FILE: &str = "settings.json";
const SESSIONS_DIR: &str = "sessions";

pub fn data_dir() -> PathBuf {
    dirs::data_dir()
        .or_else(|| dirs::home_dir().map(|home| home.join(".local/share")))
        .unwrap_or_else(std::env::temp_dir)
        .join(APP_DIR)
}

pub fn customers_file() -> PathBuf {
    data_dir().join(CUSTOMERS_FILE)
}

pub fn alerts_file() -> PathBuf {
    data_dir().join(ALERTS_FILE)
}

pub fn settings_file() -> PathBuf {
    data_dir().join(SETTINGS_FILE)
}

pub fn session_cookie_path(customer_id: u64) -> PathBuf {
    sessions_dir().join(format!("{customer_id}.cookies"))
}

pub fn session_record_path(customer_id: u64) -> PathBuf {
    sessions_dir().join(format!("{customer_id}.json"))
}

fn sessions_dir() -> PathBuf {
    data_dir().join(SESSIONS_DIR)
}
