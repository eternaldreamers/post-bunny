use serde::{Deserialize, Serialize};

use crate::config;
use crate::domain::identity::{is_valid_dni, is_valid_ruc, ruc_from_dni};
use crate::domain::message::{MessageKind, SunatMessage};
use crate::i18n::{self, Language};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum LoginType {
    Dni,
    Ruc,
}

impl LoginType {
    pub fn label(self) -> &'static str {
        match self {
            LoginType::Dni => "DNI",
            LoginType::Ruc => "RUC",
        }
    }

    pub fn description(self, language: Language) -> &'static str {
        match self {
            LoginType::Dni => i18n::t(
                language,
                "Sign in with the DNI and the SOL password.",
                "Inicia sesión con el DNI y la contraseña SOL.",
            ),
            LoginType::Ruc => i18n::t(
                language,
                "Sign in with the RUC, the SOL user and the SOL password.",
                "Inicia sesión con el RUC, el usuario SOL y la contraseña SOL.",
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AccountStatus {
    Connected,
    Pending,
    SessionExpired,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: u64,
    pub display_name: Option<String>,
    pub login_type: LoginType,
    pub dni: Option<String>,
    pub ruc: String,
    pub sol_user: Option<String>,
    #[serde(skip)]
    pub password: String,
    pub status: AccountStatus,
    pub last_sync: Option<String>,
    pub notes: String,
    pub messages: Vec<SunatMessage>,
}

impl Customer {
    pub fn title(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| format!("{} {}", self.login_type.label(), self.login_identifier()))
    }

    pub fn login_identifier(&self) -> &str {
        match self.login_type {
            LoginType::Dni => self.dni.as_deref().unwrap_or(""),
            LoginType::Ruc => &self.ruc,
        }
    }

    pub fn initials(&self) -> String {
        let Some(name) = self.display_name.as_deref() else {
            return self.login_type.label().chars().take(2).collect();
        };
        let mut parts = name.split_whitespace();
        let first = parts.next().and_then(|part| part.chars().next());
        let last = parts.next().and_then(|part| part.chars().next());
        match (first, last) {
            (Some(first), Some(last)) => {
                format!("{}{}", first.to_uppercase(), last.to_uppercase())
            }
            (Some(first), None) => first.to_uppercase().collect(),
            _ => self.login_type.label().chars().take(2).collect(),
        }
    }

    pub fn unread_count(&self) -> usize {
        self.messages.iter().filter(|message| !message.read).count()
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn notification_count(&self) -> usize {
        self.messages
            .iter()
            .filter(|message| message.kind == MessageKind::Notification)
            .count()
    }

    pub fn attachment_count(&self) -> usize {
        self.messages
            .iter()
            .map(|message| message.attachment_count)
            .sum()
    }

    pub fn account_label(&self) -> String {
        format!("CUS-{:04}", self.id)
    }

    pub fn message(&self, cod_mensaje: u64) -> Option<&SunatMessage> {
        self.messages
            .iter()
            .find(|message| message.cod_mensaje == cod_mensaje)
    }

    pub fn message_mut(&mut self, cod_mensaje: u64) -> Option<&mut SunatMessage> {
        self.messages
            .iter_mut()
            .find(|message| message.cod_mensaje == cod_mensaje)
    }

    pub fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        self.title().to_lowercase().contains(query)
            || self.ruc.to_lowercase().contains(query)
            || self.dni.as_deref().is_some_and(|dni| dni.contains(query))
            || self
                .sol_user
                .as_deref()
                .is_some_and(|user| user.to_lowercase().contains(query))
    }

    pub fn uses_demo_password(&self) -> bool {
        self.password == config::DEMO_SOL_PASSWORD
    }

    pub fn has_real_password(&self) -> bool {
        !self.password.trim().is_empty() && !self.uses_demo_password()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomerDraft {
    pub login_type: LoginType,
    pub dni: String,
    pub ruc: String,
    pub sol_user: String,
    pub password: String,
}

impl CustomerDraft {
    pub fn from_customer(customer: &Customer) -> Self {
        match customer.login_type {
            LoginType::Dni => Self {
                login_type: LoginType::Dni,
                dni: customer.dni.clone().unwrap_or_default(),
                ruc: String::new(),
                sol_user: String::new(),
                password: customer.password.clone(),
            },
            LoginType::Ruc => Self {
                login_type: LoginType::Ruc,
                dni: String::new(),
                ruc: customer.ruc.clone(),
                sol_user: customer.sol_user.clone().unwrap_or_default(),
                password: customer.password.clone(),
            },
        }
    }

    pub fn resolved_ruc(&self) -> Option<String> {
        match self.login_type {
            LoginType::Dni => ruc_from_dni(&self.dni),
            LoginType::Ruc => is_valid_ruc(&self.ruc).then(|| self.ruc.clone()),
        }
    }

    pub fn validate(&self, language: Language) -> Result<(), &'static str> {
        match self.login_type {
            LoginType::Dni => {
                if !is_valid_dni(&self.dni) {
                    return Err(i18n::t(
                        language,
                        "Enter an 8-digit DNI.",
                        "Ingresa un DNI de 8 dígitos.",
                    ));
                }
                if self.password.trim().is_empty() {
                    return Err(i18n::t(
                        language,
                        "Enter the SOL password.",
                        "Ingresa la contraseña SOL.",
                    ));
                }
            }
            LoginType::Ruc => {
                if !is_valid_ruc(&self.ruc) {
                    return Err(i18n::t(
                        language,
                        "Enter a valid 11-digit RUC (check digit included).",
                        "Ingresa un RUC válido de 11 dígitos (con dígito verificador).",
                    ));
                }
                if self.sol_user.trim().is_empty() {
                    return Err(i18n::t(
                        language,
                        "Enter the SOL user.",
                        "Ingresa el usuario SOL.",
                    ));
                }
                if self.password.trim().is_empty() {
                    return Err(i18n::t(
                        language,
                        "Enter the SOL password.",
                        "Ingresa la contraseña SOL.",
                    ));
                }
            }
        }
        Ok(())
    }
}
