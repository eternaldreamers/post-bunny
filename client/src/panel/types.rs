use std::path::PathBuf;

use gpui::Entity;

use crate::domain::customer::LoginType;
use crate::preview::PreviewContent;
use crate::sunat::{Session, SunatCredentials};
use crate::ui::text_input::TextInput;

pub struct CustomerForm {
    pub open: bool,
    pub editing: Option<u64>,
    pub login_type: LoginType,
    pub dni: Entity<TextInput>,
    pub ruc: Entity<TextInput>,
    pub sol_user: Entity<TextInput>,
    pub password: Entity<TextInput>,
    pub show_password: bool,
    pub error: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToastKind {
    Success,
    Info,
    Danger,
}

#[derive(Clone)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub kind: ToastKind,
}

pub enum AttachmentPreviewState {
    Loading,
    Ready {
        content: PreviewContent,
        path: PathBuf,
    },
    Error(String),
}

pub struct AttachmentPreview {
    pub customer_id: u64,
    pub cod_mensaje: u64,
    pub index: usize,
    pub name: String,
    pub size: String,
    pub state: AttachmentPreviewState,
}

pub(crate) struct SyncJob {
    pub customer_id: u64,
    pub credentials: Option<SunatCredentials>,
    pub existing: Option<Session>,
    pub jar_path: PathBuf,
}
