use std::path::PathBuf;

use serde_json::Value;

use crate::config;
use crate::domain::customer::LoginType;

use super::constants::{MENU, MENU_AUTH, OAUTH, VISOR};
use super::http::CurlClient;
use super::types::{Label, Session, SunatCredentials};
use super::url::{extract_between, query_param};
use super::validation::{ensure_not_rejected, ensure_ok};

pub(crate) struct LoginResult {
    pub client: CurlClient,
    pub ruc: String,
    pub display_name: Option<String>,
    pub labels: Vec<Label>,
}

pub(crate) fn start_session(
    credentials: &SunatCredentials,
    jar_path: PathBuf,
) -> Result<Session, String> {
    let login = login_and_open(credentials, jar_path)?;
    Ok(Session {
        ruc: login.ruc,
        display_name: login.display_name,
        labels: login.labels,
        client: login.client,
    })
}

pub(crate) fn session_is_valid(session: &Session) -> bool {
    let url = format!(
        "{VISOR}/listNotiMenPag?tipoMsj=1&codCarpeta=00&codEtiqueta=&page=1&des_asunto=&codMensaje=&tipoOrden=NADA"
    );
    match session.client.get_with(&url, &[("X-Ruc", &session.ruc)]) {
        Ok(response) if response.status == 200 => response
            .json()
            .ok()
            .and_then(|value| value.get("rows").map(|rows| !rows.is_null()))
            .unwrap_or(false),
        _ => false,
    }
}

fn login_and_open(
    credentials: &SunatCredentials,
    jar_path: PathBuf,
) -> Result<LoginResult, String> {
    let client = CurlClient::persistent(jar_path)?;
    client.clear();

    let response = client.get(MENU)?;
    let menu_html = response.text();
    ensure_not_rejected(&menu_html)?;
    let authen_url = extract_between(&menu_html, "redirect(\"", "\")")
        .ok_or_else(|| "SUNAT did not return the OAuth URL (WAF?)".to_string())?;
    let state =
        query_param(authen_url, "state").ok_or_else(|| "OAuth state is missing".to_string())?;

    let mut response = client.get(authen_url)?;
    if response.status == 302 {
        let location = response.location(authen_url)?;
        response = client.get(&location)?;
    }
    if response.status != 200 {
        return Err(format!(
            "SUNAT login page returned HTTP {}",
            response.status
        ));
    }
    let login_html = response.text();

    let form = login_form(credentials, state);
    let response = client.post_form(&format!("{OAUTH}/j_security_check"), &form, &[])?;
    if response.status != 302 {
        let body = response.text();
        ensure_not_rejected(&body)?;
        if login_html.contains("txtCaptcha") || body.contains("txtCaptcha") {
            return Err(
                "SUNAT requires a captcha. Complete the login in a browser and try again later."
                    .into(),
            );
        }
        return Err(format!(
            "SUNAT rejected the login (HTTP {}). Check the document, SOL user and password.",
            response.status
        ));
    }

    let mut response = client.get(&response.location(&format!("{OAUTH}/j_security_check"))?)?;
    if response.status == 302 {
        let location = response.location(MENU)?;
        response = client.get(&location)?;
    }
    let menu_html = response.text();
    ensure_not_rejected(&menu_html)?;
    if !menu_html.contains("Bienvenido") && !menu_html.contains("Salir") {
        return Err("SUNAT login failed: the welcome page was not returned.".into());
    }
    let display_name = extract_welcome_name(&menu_html);

    let domains: Value = get_json(&client, "https://e-menu.sunat.gob.pe/a/dominios.json")?;
    let sdm = domains
        .get("items")
        .and_then(|items| items.as_array())
        .and_then(|items| items.first())
        .and_then(|item| item.get("sdm"))
        .and_then(|value| value.as_str())
        .unwrap_or("ww1");

    let response = client.get(&format!("{MENU}?action=buzon&s={sdm}"))?;
    if response.status != 302 {
        return Err(format!(
            "The SUNAT mailbox did not open (HTTP {}). The session may have expired.",
            response.status
        ));
    }
    let visor_url = response.location(MENU)?;

    let response = client.get(&visor_url)?;
    if response.status != 200 {
        return Err(format!(
            "The SUNAT viewer returned HTTP {}",
            response.status
        ));
    }
    let visor_html = response.text();
    ensure_not_rejected(&visor_html)?;
    let ruc = extract_between(&visor_html, "<meta name=\"meta-token\" content=\"", "\"")
        .ok_or_else(|| "The SUNAT viewer did not return the RUC".to_string())?;
    let labels = extract_labels(&visor_html);

    Ok(LoginResult {
        client,
        ruc: ruc.to_string(),
        display_name,
        labels,
    })
}

fn login_form(credentials: &SunatCredentials, state: String) -> Vec<(String, String)> {
    let (tipo, dni, ruc, sol_user) = match credentials.login_type {
        LoginType::Dni => (
            "1",
            credentials.dni.clone().unwrap_or_default(),
            String::new(),
            String::new(),
        ),
        LoginType::Ruc => (
            "2",
            String::new(),
            credentials.ruc.clone().unwrap_or_default(),
            credentials.sol_user.clone().unwrap_or_default(),
        ),
    };
    vec![
        ("tipo".to_string(), tipo.to_string()),
        ("dni".to_string(), dni),
        ("custom_ruc".to_string(), ruc),
        ("j_username".to_string(), sol_user),
        ("j_password".to_string(), credentials.password.clone()),
        ("captcha".to_string(), String::new()),
        ("originalUrl".to_string(), MENU_AUTH.to_string()),
        ("lang".to_string(), "es-PE".to_string()),
        ("state".to_string(), state),
    ]
}

fn get_json(client: &CurlClient, url: &str) -> Result<Value, String> {
    let response = client.get(url)?;
    ensure_ok(&response)?;
    response.json()
}

fn extract_welcome_name(html: &str) -> Option<String> {
    let index = html.find("Bienvenido")?;
    let rest = &html[index + "Bienvenido".len()..];
    let snippet = &rest[..rest.len().min(1200)];
    let text = super::html::html_to_text(snippet);
    text.lines().map(str::trim).find_map(|line| {
        let line = line
            .trim_start_matches(|character: char| {
                character == ','
                    || character == ':'
                    || character == '-'
                    || character.is_whitespace()
            })
            .trim();
        if line.is_empty() || line.eq_ignore_ascii_case("Salir") {
            None
        } else {
            Some(line.to_string())
        }
    })
}

fn extract_labels(html: &str) -> Vec<Label> {
    let Some(raw) = extract_between(html, "var listEtiquetas = $.parseJSON('", "');") else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return Vec::new();
    };
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let code = item.get("codEtiqueta")?.as_str()?.to_string();
                    let name = item
                        .get("descEtiqueta")
                        .and_then(|value| value.as_str())
                        .unwrap_or(&code)
                        .to_string();
                    let color = item
                        .get("colorEtiqueta")
                        .and_then(|value| value.as_str())
                        .and_then(|value| {
                            u32::from_str_radix(value.trim_start_matches('#'), 16).ok()
                        })
                        .unwrap_or(config::DEFAULT_LABEL_COLOR);
                    Some(Label { code, name, color })
                })
                .collect()
        })
        .unwrap_or_default()
}
