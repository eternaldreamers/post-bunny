use std::collections::BTreeMap;

use chrono::NaiveDateTime;
use serde_json::Value;

use crate::domain::message::{MessageKind, MessageLabel, SunatMessage};

use super::constants::{MAX_PAGES, VISOR};
use super::http::CurlClient;
use super::types::{Label, Session};
use super::validation::ensure_ok;

struct ListContext {
    tipo_msj: String,
    cod_carpeta: String,
    cod_etiqueta: String,
}

pub(crate) fn fetch_messages(session: &Session) -> Result<Vec<SunatMessage>, String> {
    let client = &session.client;
    let ruc = &session.ruc;
    let labels = &session.labels;

    let folders = list_folders(client, ruc)?;
    let contexts = build_contexts(&folders, labels);

    let mut rows: BTreeMap<u64, (Value, String)> = BTreeMap::new();
    for context in contexts {
        for (code, row, context_tipo) in list_context(client, ruc, &context)? {
            rows.insert(code, (row, context_tipo));
        }
    }

    let mut messages = rows
        .into_values()
        .filter_map(|(row, context_tipo)| row_to_message(&row, &context_tipo, labels))
        .collect::<Vec<_>>();
    messages.sort_by_key(|message| std::cmp::Reverse(fec_key(&message.fec_publica)));
    Ok(messages)
}

fn build_contexts(folders: &[String], labels: &[Label]) -> Vec<ListContext> {
    let mut contexts = vec![
        ListContext {
            tipo_msj: "2".into(),
            cod_carpeta: "00".into(),
            cod_etiqueta: String::new(),
        },
        ListContext {
            tipo_msj: "1".into(),
            cod_carpeta: "00".into(),
            cod_etiqueta: String::new(),
        },
    ];
    contexts.extend(folders.iter().map(|folder| ListContext {
        tipo_msj: String::new(),
        cod_carpeta: folder.clone(),
        cod_etiqueta: String::new(),
    }));
    contexts.extend(labels.iter().map(|label| ListContext {
        tipo_msj: String::new(),
        cod_carpeta: String::new(),
        cod_etiqueta: label.code.clone(),
    }));
    contexts
}

fn list_folders(client: &CurlClient, ruc: &str) -> Result<Vec<String>, String> {
    let response = client.get_with(&format!("{VISOR}/ajax/listarCarpetas"), &[("X-Ruc", ruc)])?;
    ensure_ok(&response)?;
    let value: Value = response.json()?;
    Ok(value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    item.get("codCarpeta")
                        .and_then(|value| value.as_str())
                        .map(str::to_string)
                })
                .collect()
        })
        .unwrap_or_default())
}

fn list_context(
    client: &CurlClient,
    ruc: &str,
    context: &ListContext,
) -> Result<Vec<(u64, Value, String)>, String> {
    let mut found = Vec::new();
    for page in 1..=MAX_PAGES {
        let url = format!(
            "{VISOR}/listNotiMenPag?tipoMsj={}&codCarpeta={}&codEtiqueta={}&page={}&des_asunto=&codMensaje=&tipoOrden=NADA",
            urlencoding::encode(&context.tipo_msj),
            urlencoding::encode(&context.cod_carpeta),
            urlencoding::encode(&context.cod_etiqueta),
            page
        );
        let response = client.get_with(&url, &[("X-Ruc", ruc)])?;
        ensure_ok(&response)?;
        let value: Value = response.json()?;
        let rows = match value.get("rows") {
            Some(Value::Null) => {
                return Err("The SUNAT viewer session expired (rows = null). Sync again.".into())
            }
            Some(Value::Array(rows)) => rows,
            _ => return Err("Unexpected response from listNotiMenPag".into()),
        };
        if rows.is_empty() {
            break;
        }
        for row in rows {
            let Some(code) = row.get("codMensaje").and_then(Value::as_u64) else {
                continue;
            };
            let context_tipo = context_tipo_for(row, context);
            found.push((code, row.clone(), context_tipo));
        }
    }
    Ok(found)
}

fn context_tipo_for(row: &Value, context: &ListContext) -> String {
    if !context.tipo_msj.is_empty() {
        context.tipo_msj.clone()
    } else if !context.cod_carpeta.is_empty() && context.cod_carpeta != "00" {
        context.cod_carpeta.clone()
    } else if !context.cod_etiqueta.is_empty() {
        context.cod_etiqueta.clone()
    } else {
        row_u64(row, "indTipmsj", 1).to_string()
    }
}

fn row_to_message(row: &Value, context_tipo: &str, labels: &[Label]) -> Option<SunatMessage> {
    let cod_mensaje = row.get("codMensaje")?.as_u64()?;
    let kind = if row_u64(row, "indTipmsj", 1) == 2 {
        MessageKind::Notification
    } else {
        MessageKind::Message
    };
    let label_code = row_str(row, "codEtiqueta", "00");
    let label = if label_code == "00" {
        None
    } else {
        labels
            .iter()
            .find(|label| label.code == label_code)
            .map(|label| MessageLabel {
                name: label.name.clone(),
                color: label.color,
            })
    };

    Some(SunatMessage {
        cod_mensaje,
        kind,
        subject: row_str(row, "desAsunto", "(no subject)"),
        body: String::new(),
        body_loaded: false,
        context_tipo: context_tipo.to_string(),
        fec_publica: row_str(row, "fecPublica", ""),
        fec_envio: row_str(row, "fecEnvio", ""),
        vigencia: row
            .get("fecVigencia")
            .and_then(Value::as_str)
            .map(|value| value.split_whitespace().next().unwrap_or(value).to_string()),
        read: row_u64(row, "indEstado", 0) == 1,
        urgent: row_u64(row, "indUrg", 0) == 1,
        attachment_count: row_u64(row, "cantidadArchAdj", 0) as usize,
        attachments: Vec::new(),
        label,
    })
}

fn row_str(row: &Value, key: &str, fallback: &str) -> String {
    row.get(key)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn row_u64(row: &Value, key: &str, fallback: u64) -> u64 {
    row.get(key).and_then(Value::as_u64).unwrap_or(fallback)
}

fn fec_key(value: &str) -> i64 {
    NaiveDateTime::parse_from_str(value, "%d/%m/%Y %H:%M:%S")
        .map(|date| date.and_utc().timestamp())
        .unwrap_or(0)
}
