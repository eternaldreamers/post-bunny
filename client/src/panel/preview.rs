use gpui::{ClickEvent, Context, Window};

use crate::domain::message::Attachment;
use crate::i18n;
use crate::preview;
use crate::sunat;

use super::{AttachmentPreview, AttachmentPreviewState, Panel, ToastKind};

impl Panel {
    pub fn open_attachment_preview(
        &mut self,
        customer_id: u64,
        cod_mensaje: u64,
        index: usize,
        attachment: Attachment,
        cx: &mut Context<Self>,
    ) {
        if !attachment.downloadable {
            self.show_toast(
                i18n::t(
                    self.language,
                    "This document is generated inside the message body.",
                    "Este documento se genera dentro del cuerpo del mensaje.",
                ),
                ToastKind::Info,
                cx,
            );
            return;
        }
        let Some(session) = self.sessions.get(&customer_id).cloned() else {
            self.show_toast(
                i18n::t(
                    self.language,
                    "Sync this customer to download and preview attachments.",
                    "Sincroniza este cliente para descargar y previsualizar adjuntos.",
                ),
                ToastKind::Info,
                cx,
            );
            return;
        };
        let Some(message) = self
            .customer(customer_id)
            .and_then(|customer| customer.message(cod_mensaje))
        else {
            return;
        };
        let context_tipo = message.context_tipo.clone();
        self.attachment_preview = Some(AttachmentPreview {
            customer_id,
            cod_mensaje,
            index,
            name: attachment.name.clone(),
            size: attachment.size.clone(),
            state: AttachmentPreviewState::Loading,
        });
        cx.notify();

        let executor = cx.background_executor().clone();
        let task = executor.spawn(async move {
            let downloaded = sunat::fetch_attachment(&session, cod_mensaje, &context_tipo, index)?;
            preview::prepare(downloaded)
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |panel, cx| {
                let Some(current) = panel.attachment_preview.as_mut() else {
                    return;
                };
                if current.cod_mensaje != cod_mensaje || current.index != index {
                    return;
                }
                current.state = match result {
                    Ok(prepared) => AttachmentPreviewState::Ready {
                        content: prepared.content,
                        path: prepared.path,
                    },
                    Err(error) => AttachmentPreviewState::Error(error),
                };
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn download_attachment_preview(
        &mut self,
        _: &ClickEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(preview) = self.attachment_preview.as_ref() else {
            return;
        };
        let AttachmentPreviewState::Ready { path, .. } = &preview.state else {
            return;
        };
        let path = path.clone();
        let name = preview.name.clone();
        match preview::download_to_downloads(&path, &name) {
            Ok(target) => {
                let filename = target
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or(&name);
                self.show_toast(
                    i18n::downloaded_attachment(self.language, filename),
                    ToastKind::Success,
                    cx,
                );
            }
            Err(error) => {
                self.show_toast(
                    format!(
                        "{} {error}",
                        i18n::t(
                            self.language,
                            "Could not download the attachment:",
                            "No se pudo descargar el adjunto:",
                        )
                    ),
                    ToastKind::Danger,
                    cx,
                );
            }
        }
    }

    pub fn close_attachment_preview(
        &mut self,
        _: &ClickEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.attachment_preview = None;
        cx.notify();
    }
}
