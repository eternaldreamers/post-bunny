use serde_json::Value;

use super::url::absolute_url;

pub(crate) struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .rev()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }

    pub fn location(&self, base: &str) -> Result<String, String> {
        self.header("location")
            .map(|location| absolute_url(base, location))
            .ok_or_else(|| format!("Missing Location header (HTTP {})", self.status))
    }

    pub fn json(&self) -> Result<Value, String> {
        serde_json::from_slice(&self.body)
            .map_err(|error| format!("Invalid JSON from SUNAT: {error}"))
    }
}

pub(crate) fn parse_headers(text: &str) -> (u16, Vec<(String, String)>) {
    let mut status = 0;
    let mut headers = Vec::new();
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.starts_with("HTTP/") {
            status = line
                .split_whitespace()
                .nth(1)
                .and_then(|value| value.parse().ok())
                .unwrap_or(0);
            headers.clear();
        } else if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim().to_ascii_lowercase(), value.trim().to_string()));
        }
    }
    (status, headers)
}
