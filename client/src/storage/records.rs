use serde::{Deserialize, Serialize};

use crate::domain::alert::Alert;
use crate::domain::customer::Customer;
use crate::i18n::Language;

use super::json::{read_json, write_json};
use super::paths::{alerts_file, customers_file, settings_file};

const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct CustomersFile {
    version: u32,
    customers: Vec<Customer>,
}

#[derive(Serialize, Deserialize)]
struct AlertsFile {
    version: u32,
    alerts: Vec<Alert>,
}

#[derive(Serialize, Deserialize)]
struct SettingsFile {
    version: u32,
    #[serde(default)]
    language: Language,
}

pub fn customers_file_exists() -> bool {
    customers_file().exists()
}

pub fn alerts_file_exists() -> bool {
    alerts_file().exists()
}

pub fn load_customers() -> Vec<Customer> {
    read_json::<CustomersFile>(&customers_file())
        .map(|file| file.customers)
        .unwrap_or_default()
}

pub fn save_customers(customers: &[Customer]) -> Result<(), String> {
    write_json(
        &customers_file(),
        &CustomersFile {
            version: VERSION,
            customers: customers.to_vec(),
        },
    )
}

pub fn load_alerts() -> Vec<Alert> {
    read_json::<AlertsFile>(&alerts_file())
        .map(|file| file.alerts)
        .unwrap_or_default()
}

pub fn save_alerts(alerts: &[Alert]) -> Result<(), String> {
    write_json(
        &alerts_file(),
        &AlertsFile {
            version: VERSION,
            alerts: alerts.to_vec(),
        },
    )
}

pub fn load_language() -> Language {
    read_json::<SettingsFile>(&settings_file())
        .map(|settings| settings.language)
        .unwrap_or_default()
}

pub fn save_language(language: Language) -> Result<(), String> {
    write_json(
        &settings_file(),
        &SettingsFile {
            version: VERSION,
            language,
        },
    )
}
