use gpui::Context;

use crate::domain::alert::AlertState;
use crate::domain::navigation::DetailTab;
use crate::i18n;

use super::{Panel, ToastKind};

impl Panel {
    pub fn resolve_alert(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some(alert) = self.alerts.iter_mut().find(|alert| alert.id == id) {
            alert.state = AlertState::Resolved;
        }
        self.persist_alerts();
        self.show_toast(
            i18n::t(self.language, "Alert resolved.", "Alerta resuelta."),
            ToastKind::Success,
            cx,
        );
    }

    pub fn resolve_alert_for_message(&mut self, customer_id: u64, cod_mensaje: u64) {
        for alert in &mut self.alerts {
            if alert.customer_id == customer_id && alert.cod_mensaje == Some(cod_mensaje) {
                alert.state = AlertState::Resolved;
            }
        }
        self.persist_alerts();
    }

    pub fn open_alert(&mut self, id: u64, cx: &mut Context<Self>) {
        let Some(alert) = self.alerts.iter().find(|alert| alert.id == id).cloned() else {
            return;
        };
        if let Some(alert) = self.alerts.iter_mut().find(|alert| alert.id == id) {
            alert.state = AlertState::Resolved;
        }
        self.persist_alerts();
        match alert.cod_mensaje {
            Some(cod_mensaje) => self.jump_to_message(alert.customer_id, cod_mensaje, cx),
            None => {
                self.open_customer_by_id(alert.customer_id, cx);
                self.detail_tab = DetailTab::Inbox;
            }
        }
        cx.notify();
    }
}
