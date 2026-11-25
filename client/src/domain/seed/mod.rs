mod accounting;
mod alerts;
mod builders;
mod company;
mod customers;
mod personal;

use crate::domain::alert::Alert;
use crate::domain::customer::Customer;

pub fn seed_customers() -> Vec<Customer> {
    customers::customers()
}

pub fn seed_alerts() -> Vec<Alert> {
    alerts::alerts()
}
