use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use super::constants::{CHROME_UA, CURL_TIMEOUT_SECONDS};
use super::response::{parse_headers, Response};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub(crate) struct CurlClient {
    dir: PathBuf,
    jar: PathBuf,
}

#[derive(Clone, Copy)]
enum HttpMethod {
    Get,
    Post,
}

impl CurlClient {
    pub fn persistent(jar: PathBuf) -> Result<Self, String> {
        let dir = jar
            .parent()
            .map(|parent| parent.to_path_buf())
            .unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&dir)
            .map_err(|error| format!("Could not create cookie dir: {error}"))?;
        Ok(Self { dir, jar })
    }

    pub fn clear(&self) {
        let _ = fs::remove_file(&self.jar);
    }

    pub fn get(&self, url: &str) -> Result<Response, String> {
        self.request(url, HttpMethod::Get, &[], None)
    }

    pub fn get_with(&self, url: &str, headers: &[(&str, &str)]) -> Result<Response, String> {
        self.request(url, HttpMethod::Get, headers, None)
    }

    pub fn post_form(
        &self,
        url: &str,
        form: &[(String, String)],
        headers: &[(&str, &str)],
    ) -> Result<Response, String> {
        self.request(url, HttpMethod::Post, headers, Some(form))
    }

    fn request(
        &self,
        url: &str,
        method: HttpMethod,
        headers: &[(&str, &str)],
        form: Option<&[(String, String)]>,
    ) -> Result<Response, String> {
        let id = REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let header_path = self.dir.join(format!("headers-{id}.txt"));
        let body_path = self.dir.join(format!("body-{id}.bin"));

        let mut command = Command::new("curl");
        command
            .arg("-sS")
            .arg("--max-time")
            .arg(CURL_TIMEOUT_SECONDS)
            .arg("--compressed")
            .arg("--cookie-jar")
            .arg(&self.jar)
            .arg("--cookie")
            .arg(&self.jar)
            .arg("--user-agent")
            .arg(CHROME_UA)
            .arg("--dump-header")
            .arg(&header_path)
            .arg("--output")
            .arg(&body_path);

        if matches!(method, HttpMethod::Post) {
            command.arg("-X").arg("POST");
        }

        if let Some(form) = form {
            let body = form
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}={}",
                        urlencoding::encode(key),
                        urlencoding::encode(value)
                    )
                })
                .collect::<Vec<_>>()
                .join("&");
            command
                .arg("-H")
                .arg("content-type: application/x-www-form-urlencoded")
                .arg("--data-binary")
                .arg(body);
        }

        for (key, value) in headers {
            command.arg("-H").arg(format!("{key}: {value}"));
        }
        command.arg(url);

        let output = command.output();
        let cleanup = || {
            let _ = fs::remove_file(&header_path);
            let _ = fs::remove_file(&body_path);
        };

        let output = match output {
            Ok(output) => output,
            Err(error) => {
                cleanup();
                return Err(format!("Could not run curl: {error}"));
            }
        };
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            cleanup();
            return Err(format!("curl failed: {}", stderr.trim()));
        }

        let header_text = fs::read_to_string(&header_path).unwrap_or_default();
        let (status, headers) = parse_headers(&header_text);
        let body = fs::read(&body_path).unwrap_or_default();
        cleanup();

        Ok(Response {
            status,
            headers,
            body,
        })
    }
}
