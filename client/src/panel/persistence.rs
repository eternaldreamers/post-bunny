use crate::storage;

use super::Panel;

impl Panel {
    pub(super) fn persist_customers(&self) {
        if let Err(error) = storage::save_customers(&self.customers) {
            eprintln!("Could not persist customers: {error}");
        }
    }

    pub(super) fn persist_alerts(&self) {
        if let Err(error) = storage::save_alerts(&self.alerts) {
            eprintln!("Could not persist alerts: {error}");
        }
    }
}
