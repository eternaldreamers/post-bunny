use gpui::{div, prelude::*, px, rgb, Div, FontWeight, SharedString};

use crate::ui::theme;

use super::icon::icon;

pub fn element_id(value: impl Into<SharedString>) -> SharedString {
    value.into()
}

pub fn card() -> Div {
    div()
        .flex()
        .flex_col()
        .bg(rgb(theme::SURFACE))
        .border_1()
        .border_color(rgb(theme::BORDER_SOFT))
        .rounded_xl()
        .overflow_hidden()
}

pub fn avatar(initials: &str, seed: u64, size: f32) -> Div {
    let color = theme::avatar_color(seed);
    div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded_full()
        .bg(theme::tint(color, 0.16))
        .border_1()
        .border_color(theme::tint(color, 0.24))
        .text_color(rgb(color))
        .text_size(px(if size >= 44. {
            14.
        } else if size >= 32. {
            11.5
        } else {
            10.
        }))
        .font_weight(FontWeight::SEMIBOLD)
        .child(initials.to_string())
}

pub fn empty_state(icon_name: &str, title: &str, body: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .py_12()
        .px_6()
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(44.))
                .rounded_full()
                .bg(rgb(theme::SURFACE_2))
                .child(icon(icon_name, 20., theme::FAINT)),
        )
        .child(
            div()
                .text_size(px(14.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme::TEXT))
                .child(title.to_string()),
        )
        .child(
            div()
                .max_w(px(320.))
                .text_center()
                .text_size(px(12.))
                .text_color(rgb(theme::MUTED))
                .child(body.to_string()),
        )
}
