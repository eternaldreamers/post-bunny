use serde_json::Value;

use crate::domain::message::Attachment;

use super::constants::VISOR;
use super::html::html_to_text;
use super::types::{DownloadedAttachment, MessageDetail, Session};
use super::url::sanitize_url;
use super::validation::ensure_ok;

pub fn fetch_message_detail(
    session: &Session,
    cod_mensaje: u64,
    context_tipo: &str,
) -> Result<MessageDetail, String> {
    let value = detail_json(session, cod_mensaje, context_tipo)?;
    let attachments = value
        .get("listAttach")
        .and_then(Value::as_array)
        .map(|items| items.iter().map(attachment_from_value).collect())
        .unwrap_or_default();
    let body = message_body(session, &value)?;

    Ok(MessageDetail { body, attachments })
}

pub fn fetch_attachment(
    session: &Session,
    cod_mensaje: u64,
    context_tipo: &str,
    index: usize,
) -> Result<DownloadedAttachment, String> {
    let detail = detail_json(session, cod_mensaje, context_tipo)?;
    let attachment = detail
        .get("listAttach")
        .and_then(Value::as_array)
        .and_then(|items| items.get(index))
        .ok_or_else(|| "The attachment is no longer available in SUNAT.".to_string())?;
    let indicator = attachment
        .get("indMensaje")
        .and_then(Value::as_str)
        .unwrap_or("");
    if indicator != "1" && indicator != "2" {
        return Err(
            "This document is generated inside the message body and cannot be downloaded separately."
                .into(),
        );
    }

    let cod_archivo = attachment
        .get("codArchivo")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let sistema = detail.get("sistema").and_then(Value::as_u64).unwrap_or(0);
    let anexo = if detail
        .get("tieneDocumentosAnexos")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        1
    } else {
        0
    };
    let url = format!(
        "{VISOR}/bajarArchivo/{cod_archivo}/{anexo}/{sistema}/{}",
        session.ruc
    );
    let response = session.client.get_with(&url, &[("X-Ruc", &session.ruc)])?;
    if response.status != 200 {
        return Err(format!(
            "SUNAT returned HTTP {} while downloading the attachment.",
            response.status
        ));
    }
    if response.body.is_empty() {
        return Err("SUNAT returned an empty attachment. Open the message again and retry.".into());
    }

    let content_type = response
        .header("content-type")
        .unwrap_or("application/octet-stream")
        .to_string();
    let filename = attachment
        .get("nomArchivo")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            response
                .header("content-disposition")
                .and_then(parse_disposition_filename)
        })
        .unwrap_or_else(|| "attachment".into());

    Ok(DownloadedAttachment {
        bytes: response.body,
        content_type,
        filename,
    })
}

fn detail_json(session: &Session, cod_mensaje: u64, context_tipo: &str) -> Result<Value, String> {
    let url = format!(
        "{VISOR}/obtenerDetalleNotiMen?codigoMensaje={cod_mensaje}&tipoMsj={}",
        urlencoding::encode(context_tipo)
    );
    let response = session.client.get_with(&url, &[("X-Ruc", &session.ruc)])?;
    ensure_ok(&response)?;
    let value: Value = response.json()?;
    if value.get("rows").is_some_and(Value::is_null) {
        return Err("The SUNAT viewer session expired (rows = null). Sync again.".into());
    }
    Ok(value)
}

fn attachment_from_value(item: &Value) -> Attachment {
    let name = item
        .get("nomArchivo")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            item.get("numId")
                .and_then(Value::as_u64)
                .map(|id| format!("Generated document {id}"))
        })
        .unwrap_or_else(|| "Attachment".into());
    let size = item
        .get("tamanoArchivoFormat")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            item.get("cntTamarch")
                .and_then(Value::as_u64)
                .map(format_bytes)
        })
        .unwrap_or_default();
    let indicator = item.get("indMensaje").and_then(Value::as_str).unwrap_or("");
    Attachment {
        name,
        size,
        generated: indicator == "3",
        downloadable: indicator == "1" || indicator == "2",
    }
}

fn message_body(session: &Session, value: &Value) -> Result<String, String> {
    let mut body = String::new();
    if let Some(path) = value.get("url").and_then(Value::as_str) {
        if !path.is_empty() {
            let body_url = if path.starts_with("http") {
                path.to_string()
            } else {
                format!("https://ww1.sunat.gob.pe{path}")
            };
            let body_url = sanitize_url(&body_url);
            let response = session.client.get(&body_url)?;
            body = if response.status == 200 {
                html_to_text(&response.text())
            } else {
                format!(
                    "SUNAT returned HTTP {} while loading the message body.",
                    response.status
                )
            };
        }
    }
    if body.trim().is_empty() {
        if let Some(raw) = value.get("msjMensaje").and_then(Value::as_str) {
            body = match serde_json::from_str::<Value>(raw) {
                Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_else(|_| raw.to_string()),
                Err(_) => raw.to_string(),
            };
        }
    }
    if body.trim().is_empty() {
        body = "This message has no body. Open it in SUNAT to view the original document.".into();
    }
    Ok(body)
}

fn parse_disposition_filename(header: &str) -> Option<String> {
    header
        .split(';')
        .find_map(|part| part.trim().strip_prefix("filename="))
        .map(|value| value.trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}
