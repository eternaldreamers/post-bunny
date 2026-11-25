use crate::domain::alert::{Alert, AlertState, Severity};

use super::builders::{alert, AlertSeed};

pub(crate) fn alerts() -> Vec<Alert> {
    vec![
        alert(AlertSeed {
            id: 1,
            customer_id: 2,
            cod_mensaje: Some(1120031980),
            title: "Notificación de fiscalización - período 202609",
            severity: Severity::Critical,
            state: AlertState::Open,
            minutes_ago: 42,
            source: "Buzón",
        }),
        alert(AlertSeed {
            id: 2,
            customer_id: 1,
            cod_mensaje: Some(1115941439),
            title: "Generación de Registro RVIE Y RCE del período 202610",
            severity: Severity::Warning,
            state: AlertState::Open,
            minutes_ago: 180,
            source: "Buzón",
        }),
        alert(AlertSeed {
            id: 3,
            customer_id: 2,
            cod_mensaje: Some(1120034411),
            title: "Propuesta del Registro de Compras y Ventas - 202610",
            severity: Severity::Warning,
            state: AlertState::Open,
            minutes_ago: 300,
            source: "SIRE",
        }),
        alert(AlertSeed {
            id: 4,
            customer_id: 1,
            cod_mensaje: Some(1115941116),
            title: "Inicio de generación de RVIE Y RCE período 202610",
            severity: Severity::Warning,
            state: AlertState::Open,
            minutes_ago: 330,
            source: "Buzón",
        }),
        alert(AlertSeed {
            id: 5,
            customer_id: 4,
            cod_mensaje: Some(1119942110),
            title: "Constancia de Recepción RVIE - 202609",
            severity: Severity::Warning,
            state: AlertState::Open,
            minutes_ago: 1_440,
            source: "Buzón",
        }),
        alert(AlertSeed {
            id: 6,
            customer_id: 1,
            cod_mensaje: Some(1109477708),
            title: "Propuesta del Registro de Compras y Ventas - 202610",
            severity: Severity::Info,
            state: AlertState::Resolved,
            minutes_ago: 2_880,
            source: "SIRE",
        }),
    ]
}
