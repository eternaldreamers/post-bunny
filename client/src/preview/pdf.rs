use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn render_first_page(pdf: &Path, directory: &Path) -> Option<PathBuf> {
    let stem = pdf
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("document");
    let image = directory.join(format!("{stem}-preview.png"));

    let sips = Command::new("sips")
        .args(["-s", "format", "png"])
        .arg(pdf)
        .arg("--out")
        .arg(&image)
        .status();
    if sips.map(|status| status.success()).unwrap_or(false) && is_non_empty(&image) {
        return Some(image);
    }

    let prefix = directory.join(stem);
    let pdftoppm = Command::new("pdftoppm")
        .args(["-png", "-r", "110", "-f", "1", "-singlefile"])
        .arg(pdf)
        .arg(&prefix)
        .status();
    let poppler_image = directory.join(format!("{stem}.png"));
    if pdftoppm.map(|status| status.success()).unwrap_or(false) && is_non_empty(&poppler_image) {
        return Some(poppler_image);
    }

    None
}

fn is_non_empty(path: &Path) -> bool {
    path.metadata()
        .map(|metadata| metadata.len() > 0)
        .unwrap_or(false)
}
