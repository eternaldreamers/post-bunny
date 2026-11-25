use serde::{Deserialize, Serialize};

use crate::domain::message::{MessageKind, SunatMessage};
use crate::i18n::{self, Language};

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    Warning,
    Info,
}

impl Severity {
    pub fn label(self, language: Language) -> &'static str {
        match self {
            Severity::Critical => i18n::t(language, "Critical", "Crítica"),
            Severity::Warning => i18n::t(language, "Warning", "Advertencia"),
            Severity::Info => i18n::t(language, "Info", "Información"),
        }
    }

    pub fn rank(self) -> u8 {
        match self {
            Severity::Critical => 0,
            Severity::Warning => 1,
            Severity::Info => 2,
        }
    }

    pub fn for_message(message: &SunatMessage) -> Self {
        if message.urgent || message.kind == MessageKind::Notification || message.is_fiscalization()
        {
            Severity::Critical
        } else {
            Severity::Warning
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AlertState {
    Open,
    Resolved,
}

impl AlertState {
    pub fn label(self, language: Language) -> &'static str {
        match self {
            AlertState::Open => i18n::t(language, "Open", "Abierta"),
            AlertState::Resolved => i18n::t(language, "Resolved", "Resuelta"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertFilter {
    All,
    Open,
    Resolved,
}

impl AlertFilter {
    pub const ALL: [AlertFilter; 3] = [AlertFilter::All, AlertFilter::Open, AlertFilter::Resolved];

    pub fn label(self, language: Language) -> &'static str {
        match self {
            AlertFilter::All => i18n::t(language, "All alerts", "Todas las alertas"),
            AlertFilter::Open => i18n::t(language, "Open", "Abiertas"),
            AlertFilter::Resolved => i18n::t(language, "Resolved", "Resueltas"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    pub id: u64,
    pub customer_id: u64,
    pub cod_mensaje: Option<u64>,
    pub title: String,
    pub severity: Severity,
    pub state: AlertState,
    pub minutes_ago: u32,
    pub source: String,
}

impl Alert {
    pub fn open(
        id: u64,
        customer_id: u64,
        message: &SunatMessage,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id,
            customer_id,
            cod_mensaje: Some(message.cod_mensaje),
            title: message.subject.clone(),
            severity: Severity::for_message(message),
            state: AlertState::Open,
            minutes_ago: 1,
            source: source.into(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.state == AlertState::Open
    }

    pub fn time_label(&self, language: Language) -> String {
        let minutes = self.minutes_ago.max(1);
        match language {
            Language::English => {
                if minutes < 60 {
                    format!("{minutes} min ago")
                } else if minutes < 60 * 24 {
                    format!("{} h ago", minutes / 60)
                } else {
                    format!("{} d ago", minutes / (60 * 24))
                }
            }
            Language::Spanish => {
                if minutes < 60 {
                    format!("hace {minutes} min")
                } else if minutes < 60 * 24 {
                    format!("hace {} h", minutes / 60)
                } else {
                    format!("hace {} d", minutes / (60 * 24))
                }
            }
        }
    }
}

pub fn sort_alerts(alerts: &mut [Alert]) {
    alerts.sort_by_key(|alert| {
        (
            alert.state != AlertState::Open,
            alert.severity.rank(),
            alert.minutes_ago,
        )
    });
}

pub fn filter_alerts(alerts: &[Alert], filter: AlertFilter) -> Vec<Alert> {
    let mut filtered = alerts
        .iter()
        .filter(|alert| match filter {
            AlertFilter::All => true,
            AlertFilter::Open => alert.state == AlertState::Open,
            AlertFilter::Resolved => alert.state == AlertState::Resolved,
        })
        .cloned()
        .collect::<Vec<_>>();
    sort_alerts(&mut filtered);
    filtered
}

pub fn open_alerts(alerts: &[Alert]) -> Vec<Alert> {
    let mut open = alerts
        .iter()
        .filter(|alert| alert.is_open())
        .cloned()
        .collect::<Vec<_>>();
    sort_alerts(&mut open);
    open
}
