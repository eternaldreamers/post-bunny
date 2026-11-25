use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub(crate) fn read_json<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

pub(crate) fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let directory = path
        .parent()
        .ok_or_else(|| "Invalid storage path".to_string())?;
    ensure_dir(directory)?;
    let contents = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("Could not serialize local data: {error}"))?;
    fs::write(path, contents).map_err(|error| format!("Could not write local data: {error}"))?;
    secure_file(path);
    Ok(())
}

pub(crate) fn remove_file(path: &Path) {
    let _ = fs::remove_file(path);
}

fn ensure_dir(directory: &Path) -> Result<(), String> {
    fs::create_dir_all(directory)
        .map_err(|error| format!("Could not create data directory: {error}"))?;
    secure_dir(directory);
    Ok(())
}

#[cfg(unix)]
fn secure_file(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn secure_file(_path: &Path) {}

#[cfg(unix)]
fn secure_dir(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
}

#[cfg(not(unix))]
fn secure_dir(_path: &Path) {}
