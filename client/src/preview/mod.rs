mod files;
mod pdf;

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::sunat::DownloadedAttachment;

pub use files::download_to_downloads;

static PREVIEW_COUNTER: AtomicU64 = AtomicU64::new(0);

pub enum PreviewContent {
    Pdf { image: Option<PathBuf> },
    Image(PathBuf),
    Text(String),
    Unsupported,
}

pub struct PreparedAttachment {
    pub content: PreviewContent,
    pub path: PathBuf,
}

pub fn prepare(downloaded: DownloadedAttachment) -> Result<PreparedAttachment, String> {
    let id = PREVIEW_COUNTER.fetch_add(1, Ordering::Relaxed);
    let directory =
        std::env::temp_dir().join(format!("post-bunny-preview-{}-{id}", std::process::id()));
    fs::create_dir_all(&directory)
        .map_err(|error| format!("Could not create preview directory: {error}"))?;

    let path = directory.join(files::sanitize_filename(&downloaded.filename));
    fs::write(&path, &downloaded.bytes)
        .map_err(|error| format!("Could not write the downloaded attachment: {error}"))?;

    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let content_type = downloaded.content_type.to_ascii_lowercase();

    if extension == "pdf" || content_type.contains("pdf") {
        let image = pdf::render_first_page(&path, &directory);
        return Ok(PreparedAttachment {
            content: PreviewContent::Pdf { image },
            path,
        });
    }

    if files::is_image(&extension) || content_type.starts_with("image/") {
        return Ok(PreparedAttachment {
            content: PreviewContent::Image(path.clone()),
            path,
        });
    }

    if files::is_text(&extension)
        || content_type.starts_with("text/")
        || content_type.contains("xml")
        || content_type.contains("json")
    {
        let bytes = fs::read(&path).unwrap_or_default();
        let text = String::from_utf8_lossy(&bytes)
            .chars()
            .take(300_000)
            .collect::<String>();
        return Ok(PreparedAttachment {
            content: PreviewContent::Text(text),
            path,
        });
    }

    Ok(PreparedAttachment {
        content: PreviewContent::Unsupported,
        path,
    })
}
