use gpui::{ClickEvent, Context, Window};

use crate::domain::date::today_label;
use crate::i18n;
use crate::storage;

use super::Panel;

impl Panel {
    pub fn toggle_language_menu(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.language_menu_open = !self.language_menu_open;
        cx.notify();
    }

    pub fn close_language_menu(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.language_menu_open = false;
        cx.notify();
    }

    pub fn set_language(
        &mut self,
        language: crate::i18n::Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.language = language;
        window.set_window_title(i18n::window_title(language));
        self.language_menu_open = false;
        self.today = today_label(language);
        self.form.error = None;
        self.customer_search.update(cx, |input, cx| {
            input.set_placeholder(i18n::search_placeholder(language), cx);
        });
        self.form.password.update(cx, |input, cx| {
            input.set_placeholder(i18n::password_placeholder(language), cx);
        });
        if let Err(error) = storage::save_language(language) {
            eprintln!("Could not persist language: {error}");
        }
        cx.notify();
    }
}
