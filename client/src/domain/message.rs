use serde::{Deserialize, Serialize};

use crate::i18n::{self, Language};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MessageKind {
    Message,
    Notification,
}

impl MessageKind {
    pub fn label(self, language: Language) -> &'static str {
        match self {
            MessageKind::Message => i18n::t(language, "Message", "Mensaje"),
            MessageKind::Notification => i18n::t(language, "Notification", "Notificación"),
        }
    }

    pub fn context_tipo(self) -> &'static str {
        match self {
            MessageKind::Message => "1",
            MessageKind::Notification => "2",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageLabel {
    pub name: String,
    pub color: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub size: String,
    pub generated: bool,
    pub downloadable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SunatMessage {
    pub cod_mensaje: u64,
    pub kind: MessageKind,
    pub subject: String,
    pub body: String,
    pub body_loaded: bool,
    pub context_tipo: String,
    pub fec_publica: String,
    pub fec_envio: String,
    pub vigencia: Option<String>,
    pub read: bool,
    pub urgent: bool,
    pub attachment_count: usize,
    pub attachments: Vec<Attachment>,
    pub label: Option<MessageLabel>,
}

impl SunatMessage {
    pub fn preview(&self) -> String {
        self.body
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("—")
            .chars()
            .take(120)
            .collect()
    }

    pub fn is_fiscalization(&self) -> bool {
        self.label
            .as_ref()
            .is_some_and(|label| label.name.to_uppercase().contains("FISCALIZACION"))
    }
}
