mod attachment_preview;
mod customer_form;
mod delete_customer;
mod language_menu;
mod modal;
mod toast;

use gpui::{AnyElement, Context};

use crate::panel::Panel;

impl Panel {
    pub(crate) fn render_overlays(
        &mut self,
        width: f32,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let mut overlays = Vec::new();
        if self.form.open {
            overlays.push(self.render_customer_form(width, cx));
        }
        if self.delete_customer.is_some() {
            overlays.push(self.render_delete_customer(width, cx));
        }
        if self.attachment_preview.is_some() {
            overlays.push(self.render_attachment_preview(width, cx));
        }
        if self.language_menu_open {
            overlays.push(self.render_language_menu(width, cx));
        }
        if self.toast.is_some() {
            overlays.push(self.render_toast(cx));
        }
        overlays
    }
}
