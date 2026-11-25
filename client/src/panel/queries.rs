use gpui::App;

use crate::domain::alert::{filter_alerts, open_alerts as sorted_open_alerts, sort_alerts, Alert};
use crate::domain::customer::Customer;

use super::Panel;

impl Panel {
    pub fn customer(&self, id: u64) -> Option<&Customer> {
        self.customers.iter().find(|customer| customer.id == id)
    }

    pub fn customer_mut(&mut self, id: u64) -> Option<&mut Customer> {
        self.customers.iter_mut().find(|customer| customer.id == id)
    }

    pub fn selected_customer(&self) -> Option<&Customer> {
        self.selected_customer_id.and_then(|id| self.customer(id))
    }

    pub fn filtered_customers(&self, cx: &App) -> Vec<Customer> {
        let query = self.customer_search.read(cx).text().to_lowercase();
        let mut customers = self
            .customers
            .iter()
            .filter(|customer| customer.matches_query(&query))
            .cloned()
            .collect::<Vec<_>>();
        customers.sort_by_key(|customer| customer.title().to_lowercase());
        customers
    }

    pub fn open_alerts(&self) -> Vec<Alert> {
        sorted_open_alerts(&self.alerts)
    }

    pub fn filtered_alerts(&self) -> Vec<Alert> {
        filter_alerts(&self.alerts, self.alert_filter)
    }

    pub fn customer_alerts(&self, customer_id: u64) -> Vec<Alert> {
        let mut alerts = self
            .alerts
            .iter()
            .filter(|alert| alert.customer_id == customer_id)
            .cloned()
            .collect::<Vec<_>>();
        sort_alerts(&mut alerts);
        alerts
    }

    pub fn open_alerts_count(&self) -> usize {
        self.alerts.iter().filter(|alert| alert.is_open()).count()
    }

    pub fn unread_message_count(&self) -> usize {
        self.customers.iter().map(Customer::unread_count).sum()
    }

    pub fn total_message_count(&self) -> usize {
        self.customers.iter().map(Customer::message_count).sum()
    }

    pub fn attachment_count(&self) -> usize {
        self.customers.iter().map(Customer::attachment_count).sum()
    }
}
