use gpui::{App, Context, FocusHandle, KeyDownEvent, Window};

use crate::domain::customer::LoginType;
use crate::domain::navigation::Page;

use super::Panel;

impl Panel {
    pub(super) fn on_root_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();
        let modifiers = event.keystroke.modifiers;

        if key == "escape" {
            let handled = self.dismiss_top_overlay();
            if handled {
                window.focus(&self.root_focus);
                cx.notify();
            }
            return;
        }

        if key == "enter" && self.form.open {
            self.submit_customer(window, cx);
            return;
        }
        if key == "tab" && self.form.open {
            self.focus_form_field(modifiers.shift, window, cx);
            return;
        }

        if modifiers.secondary() {
            match key {
                "1" => self.navigate(Page::Home, cx),
                "2" => {
                    self.selected_customer_id = None;
                    self.navigate(Page::Customers, cx);
                }
                "3" => self.navigate(Page::Alerts, cx),
                "r" => self.sync_all_now(cx),
                "n" => self.begin_new_customer(window, cx),
                _ => {}
            }
        }
    }

    fn dismiss_top_overlay(&mut self) -> bool {
        if self.language_menu_open {
            self.language_menu_open = false;
        } else if self.attachment_preview.is_some() {
            self.attachment_preview = None;
        } else if self.form.open {
            self.form.open = false;
            self.form.error = None;
        } else if self.delete_customer.is_some() {
            self.delete_customer = None;
        } else if self.drawer_open {
            self.drawer_open = false;
        } else if self.reading_message_id.is_some() {
            self.reading_message_id = None;
        } else if self.selected_customer_id.is_some() {
            self.selected_customer_id = None;
        } else {
            return false;
        }
        true
    }

    pub(super) fn form_focus_handles(&self, cx: &App) -> Vec<FocusHandle> {
        match self.form.login_type {
            LoginType::Dni => vec![
                self.form.dni.read(cx).focus_handle.clone(),
                self.form.password.read(cx).focus_handle.clone(),
            ],
            LoginType::Ruc => vec![
                self.form.ruc.read(cx).focus_handle.clone(),
                self.form.sol_user.read(cx).focus_handle.clone(),
                self.form.password.read(cx).focus_handle.clone(),
            ],
        }
    }

    pub(super) fn focus_form_field(&self, backwards: bool, window: &mut Window, cx: &App) {
        let handles = self.form_focus_handles(cx);
        let current = handles.iter().position(|handle| handle.is_focused(window));
        let next = match (current, backwards) {
            (Some(index), false) => (index + 1) % handles.len(),
            (Some(index), true) => (index + handles.len() - 1) % handles.len(),
            (None, _) => 0,
        };
        window.focus(&handles[next]);
    }
}
