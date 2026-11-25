use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::customer::LoginType;
use crate::domain::message::{Attachment, SunatMessage};

use super::http::CurlClient;

#[derive(Clone)]
pub struct SunatCredentials {
    pub login_type: LoginType,
    pub dni: Option<String>,
    pub ruc: Option<String>,
    pub sol_user: Option<String>,
    pub password: String,
}

#[derive(Clone)]
pub struct Session {
    pub ruc: String,
    pub display_name: Option<String>,
    pub(crate) labels: Vec<Label>,
    pub(crate) client: CurlClient,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub ruc: String,
    pub display_name: Option<String>,
    labels: Vec<Label>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Label {
    pub code: String,
    pub name: String,
    pub color: u32,
}

pub struct SyncOutcome {
    pub customer_id: u64,
    pub ruc: String,
    pub display_name: Option<String>,
    pub messages: Vec<SunatMessage>,
    pub last_sync: String,
    pub session: Option<Session>,
    pub error: Option<String>,
}

pub struct MessageDetail {
    pub body: String,
    pub attachments: Vec<Attachment>,
}

pub struct DownloadedAttachment {
    pub bytes: Vec<u8>,
    pub content_type: String,
    pub filename: String,
}

impl Session {
    pub fn record(&self) -> SessionRecord {
        SessionRecord {
            ruc: self.ruc.clone(),
            display_name: self.display_name.clone(),
            labels: self.labels.clone(),
        }
    }

    pub fn from_record(record: SessionRecord, jar_path: PathBuf) -> Result<Self, String> {
        Ok(Self {
            ruc: record.ruc,
            display_name: record.display_name,
            labels: record.labels,
            client: CurlClient::persistent(jar_path)?,
        })
    }
}
