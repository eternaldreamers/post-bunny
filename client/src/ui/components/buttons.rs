use gpui::{div, prelude::*, px, rgb, Div, FontWeight, SharedString, Stateful};

use crate::ui::theme;

use super::icon::icon;

pub fn primary_button(id: impl Into<SharedString>, label: &str) -> Stateful<Div> {
    button_base(id, label)
        .bg(rgb(theme::ACCENT))
        .text_color(rgb(theme::BUTTON_TEXT))
        .font_weight(FontWeight::SEMIBOLD)
        .hover(|style| style.bg(rgb(theme::ACCENT_BRIGHT)))
}

pub fn secondary_button(id: impl Into<SharedString>, label: &str) -> Stateful<Div> {
    button_base(id, label)
        .bg(rgb(theme::SURFACE_2))
        .border_1()
        .border_color(rgb(theme::BORDER))
        .text_color(rgb(theme::TEXT))
        .hover(|style| {
            style
                .bg(rgb(theme::SURFACE_3))
                .border_color(rgb(theme::BORDER_STRONG))
        })
}

pub fn ghost_button(id: impl Into<SharedString>, label: &str) -> Stateful<Div> {
    button_base(id, label)
        .text_color(rgb(theme::MUTED))
        .hover(|style| style.bg(rgb(theme::SURFACE_2)).text_color(rgb(theme::TEXT)))
}

pub fn danger_button(id: impl Into<SharedString>, label: &str) -> Stateful<Div> {
    button_base(id, label)
        .bg(rgb(theme::DANGER))
        .text_color(rgb(theme::DANGER_TEXT))
        .font_weight(FontWeight::SEMIBOLD)
        .hover(|style| style.bg(rgb(theme::DANGER_BRIGHT)))
}

pub fn icon_button(id: impl Into<SharedString>, name: &str) -> Stateful<Div> {
    div()
        .id(id.into())
        .flex()
        .items_center()
        .justify_center()
        .size(px(32.))
        .rounded_lg()
        .text_color(rgb(theme::MUTED))
        .cursor_pointer()
        .hover(|style| style.bg(rgb(theme::SURFACE_2)).text_color(rgb(theme::TEXT)))
        .child(icon(name, 16., theme::MUTED))
}

pub fn chip(id: impl Into<SharedString>, label: &str, active: bool) -> Stateful<Div> {
    div()
        .id(id.into())
        .flex()
        .items_center()
        .justify_center()
        .h(px(28.))
        .px_3()
        .rounded_full()
        .border_1()
        .text_size(px(12.))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .when(active, |this| {
            this.bg(theme::tint(theme::ACCENT, 0.14))
                .border_color(theme::tint(theme::ACCENT, 0.35))
                .text_color(rgb(theme::ACCENT_BRIGHT))
        })
        .when(!active, |this| {
            this.bg(theme::tint(theme::SURFACE_2, 0.6))
                .border_color(rgb(theme::BORDER))
                .text_color(rgb(theme::MUTED))
                .hover(|style| style.bg(rgb(theme::SURFACE_2)).text_color(rgb(theme::TEXT)))
        })
        .child(label.to_string())
}

fn button_base(id: impl Into<SharedString>, label: &str) -> Stateful<Div> {
    div()
        .id(id.into())
        .flex()
        .items_center()
        .justify_center()
        .gap_2()
        .h(px(34.))
        .px_3()
        .rounded_lg()
        .text_size(px(12.5))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .child(label.to_string())
}
