use std::fs;
use std::path::{Path, PathBuf};

pub fn download_to_downloads(source: &Path, display_name: &str) -> Result<PathBuf, String> {
    let directory = dirs::download_dir()
        .or_else(|| dirs::home_dir().map(|home| home.join("Downloads")))
        .ok_or_else(|| "Could not locate the Downloads folder.".to_string())?;
    fs::create_dir_all(&directory)
        .map_err(|error| format!("Could not open the Downloads folder: {error}"))?;

    let target = unique_target(&directory, &sanitize_filename(display_name));
    fs::copy(source, &target).map_err(|error| format!("Could not save the attachment: {error}"))?;
    Ok(target)
}

pub fn sanitize_filename(filename: &str) -> String {
    let name = filename
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || "._- ".contains(character) {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    let name = name.trim().trim_matches('.');
    if name.is_empty() {
        "attachment.bin".into()
    } else {
        name.to_string()
    }
}

pub fn is_image(extension: &str) -> bool {
    matches!(
        extension,
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "tif" | "tiff"
    )
}

pub fn is_text(extension: &str) -> bool {
    matches!(
        extension,
        "txt"
            | "xml"
            | "json"
            | "csv"
            | "log"
            | "md"
            | "html"
            | "htm"
            | "yml"
            | "yaml"
            | "ini"
            | "sql"
    )
}

fn unique_target(directory: &Path, name: &str) -> PathBuf {
    let name_path = Path::new(name);
    let stem = name_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("attachment");
    let extension = name_path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("");

    let mut target = directory.join(name);
    let mut counter = 1;
    while target.exists() {
        let candidate = if extension.is_empty() {
            format!("{stem} ({counter})")
        } else {
            format!("{stem} ({counter}).{extension}")
        };
        target = directory.join(candidate);
        counter += 1;
    }
    target
}
