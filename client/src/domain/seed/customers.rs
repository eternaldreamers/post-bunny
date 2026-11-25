use crate::domain::customer::{AccountStatus, Customer, LoginType};

use super::builders::{customer, CustomerSeed};

pub(crate) fn customers() -> Vec<Customer> {
    vec![
        customer(CustomerSeed {
            id: 1,
            display_name: Some("Demo DNI Customer"),
            login_type: LoginType::Dni,
            dni: Some("48703474"),
            ruc: "10487034741",
            sol_user: None,
            status: AccountStatus::Connected,
            last_sync: Some("25/11/2026 19:44"),
            notes: "Main test account. Mailbox verified against SUNAT on 25/11/2026.",
            messages: super::personal::messages(),
        }),
        customer(CustomerSeed {
            id: 2,
            display_name: Some("Andes Trading S.A.C."),
            login_type: LoginType::Ruc,
            dni: None,
            ruc: "20123456786",
            sol_user: Some("ANDESOL"),
            status: AccountStatus::Connected,
            last_sync: Some("25/11/2026 18:10"),
            notes: "Company account. Two unread documents, including a fiscalization notice.",
            messages: super::company::messages(),
        }),
        customer(CustomerSeed {
            id: 3,
            display_name: Some("Demo Pending Customer"),
            login_type: LoginType::Dni,
            dni: Some("40123456"),
            ruc: "10401234565",
            sol_user: None,
            status: AccountStatus::Pending,
            last_sync: None,
            notes: "First sync pending. Add the SOL password before connecting.",
            messages: Vec::new(),
        }),
        customer(CustomerSeed {
            id: 4,
            display_name: Some("Estudio Contable Lima"),
            login_type: LoginType::Ruc,
            dni: None,
            ruc: "20556677884",
            sol_user: Some("ECLIMA"),
            status: AccountStatus::SessionExpired,
            last_sync: Some("24/11/2026 09:15"),
            notes: "The last mailbox session expired. Reconnect with the SOL credentials.",
            messages: super::accounting::messages(),
        }),
    ]
}
