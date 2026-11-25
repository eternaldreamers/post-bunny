use super::response::Response;

pub(crate) fn ensure_ok(response: &Response) -> Result<(), String> {
    if response.status == 200 {
        return Ok(());
    }
    ensure_not_rejected(&response.text())?;
    Err(format!("SUNAT returned HTTP {}", response.status))
}

pub(crate) fn ensure_not_rejected(text: &str) -> Result<(), String> {
    if text.contains("Request Rejected") {
        return Err("SUNAT's WAF rejected the request (Request Rejected).".into());
    }
    Ok(())
}
