const SERVICE: &str = "post-bunny";

pub fn load_password(customer_id: u64) -> Option<String> {
    entry(customer_id).ok()?.get_password().ok()
}

pub fn save_password(customer_id: u64, password: &str) -> Result<(), String> {
    entry(customer_id)?
        .set_password(password)
        .map_err(|error| format!("Keystore error: {error}"))
}

pub fn delete_password(customer_id: u64) {
    if let Ok(entry) = entry(customer_id) {
        let _ = entry.delete_credential();
    }
}

fn entry(customer_id: u64) -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE, &format!("customer-{customer_id}"))
        .map_err(|error| format!("Keystore error: {error}"))
}
