use crate::config;
use crate::domain::alert::{Alert, AlertState, Severity};
use crate::domain::customer::{AccountStatus, Customer, LoginType};
use crate::domain::message::{Attachment, MessageKind, MessageLabel, SunatMessage};

pub(super) struct CustomerSeed<'a> {
    pub id: u64,
    pub display_name: Option<&'a str>,
    pub login_type: LoginType,
    pub dni: Option<&'a str>,
    pub ruc: &'a str,
    pub sol_user: Option<&'a str>,
    pub status: AccountStatus,
    pub last_sync: Option<&'a str>,
    pub notes: &'a str,
    pub messages: Vec<SunatMessage>,
}

pub(super) struct MessageSeed<'a> {
    pub cod_mensaje: u64,
    pub kind: MessageKind,
    pub subject: &'a str,
    pub body: &'a str,
    pub fec_publica: &'a str,
    pub fec_envio: &'a str,
    pub vigencia: Option<&'a str>,
    pub read: bool,
    pub urgent: bool,
    pub attachments: Vec<Attachment>,
    pub label: Option<MessageLabel>,
}

pub(super) struct AlertSeed<'a> {
    pub id: u64,
    pub customer_id: u64,
    pub cod_mensaje: Option<u64>,
    pub title: &'a str,
    pub severity: Severity,
    pub state: AlertState,
    pub minutes_ago: u32,
    pub source: &'a str,
}

pub(super) fn customer(seed: CustomerSeed<'_>) -> Customer {
    Customer {
        id: seed.id,
        display_name: seed.display_name.map(str::to_string),
        login_type: seed.login_type,
        dni: seed.dni.map(str::to_string),
        ruc: seed.ruc.to_string(),
        sol_user: seed.sol_user.map(str::to_string),
        password: config::DEMO_SOL_PASSWORD.to_string(),
        status: seed.status,
        last_sync: seed.last_sync.map(str::to_string),
        notes: seed.notes.to_string(),
        messages: seed.messages,
    }
}

pub(super) fn message(seed: MessageSeed<'_>) -> SunatMessage {
    let attachment_count = seed.attachments.len();
    SunatMessage {
        cod_mensaje: seed.cod_mensaje,
        kind: seed.kind,
        subject: seed.subject.to_string(),
        body: seed.body.to_string(),
        body_loaded: true,
        context_tipo: seed.kind.context_tipo().to_string(),
        fec_publica: seed.fec_publica.to_string(),
        fec_envio: seed.fec_envio.to_string(),
        vigencia: seed.vigencia.map(str::to_string),
        read: seed.read,
        urgent: seed.urgent,
        attachment_count,
        attachments: seed.attachments,
        label: seed.label,
    }
}

pub(super) fn attachment(name: &str, size: &str) -> Attachment {
    Attachment {
        name: name.to_string(),
        size: size.to_string(),
        generated: false,
        downloadable: true,
    }
}

pub(super) fn alert(seed: AlertSeed<'_>) -> Alert {
    Alert {
        id: seed.id,
        customer_id: seed.customer_id,
        cod_mensaje: seed.cod_mensaje,
        title: seed.title.to_string(),
        severity: seed.severity,
        state: seed.state,
        minutes_ago: seed.minutes_ago,
        source: seed.source.to_string(),
    }
}
