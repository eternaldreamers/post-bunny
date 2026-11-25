use crate::sunat::SessionRecord;

use super::json::{read_json, remove_file, write_json};
use super::paths::{session_cookie_path, session_record_path};

pub fn load_session(customer_id: u64) -> Option<SessionRecord> {
    read_json(&session_record_path(customer_id))
}

pub fn save_session(customer_id: u64, record: &SessionRecord) -> Result<(), String> {
    write_json(&session_record_path(customer_id), record)
}

pub fn delete_session(customer_id: u64) {
    remove_file(&session_record_path(customer_id));
    remove_file(&session_cookie_path(customer_id));
}
