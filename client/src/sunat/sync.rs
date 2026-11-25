use std::path::PathBuf;

use chrono::Local;

use crate::domain::message::SunatMessage;

use super::auth::{session_is_valid, start_session};
use super::mailbox::fetch_messages;
use super::types::{Session, SunatCredentials, SyncOutcome};

pub fn sync_customer(
    customer_id: u64,
    credentials: Option<&SunatCredentials>,
    existing: Option<Session>,
    jar_path: PathBuf,
) -> SyncOutcome {
    match sync_inner(credentials, existing, jar_path) {
        Ok((session, messages)) => SyncOutcome {
            customer_id,
            ruc: session.ruc.clone(),
            display_name: session.display_name.clone(),
            messages,
            last_sync: Local::now().format("%d/%m/%Y %H:%M").to_string(),
            session: Some(session),
            error: None,
        },
        Err(error) => SyncOutcome {
            customer_id,
            ruc: String::new(),
            display_name: None,
            messages: Vec::new(),
            last_sync: String::new(),
            session: None,
            error: Some(error),
        },
    }
}

fn sync_inner(
    credentials: Option<&SunatCredentials>,
    existing: Option<Session>,
    jar_path: PathBuf,
) -> Result<(Session, Vec<SunatMessage>), String> {
    let session = resolve_session(credentials, existing, jar_path)?;
    let messages = fetch_messages(&session)?;
    Ok((session, messages))
}

fn resolve_session(
    credentials: Option<&SunatCredentials>,
    existing: Option<Session>,
    jar_path: PathBuf,
) -> Result<Session, String> {
    if let Some(session) = existing {
        if session_is_valid(&session) {
            return Ok(session);
        }
    }
    let credentials = credentials.ok_or_else(|| {
        "The SUNAT session expired and no credentials are available to renew it.".to_string()
    })?;
    start_session(credentials, jar_path)
}
