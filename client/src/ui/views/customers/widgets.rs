use gpui::{div, prelude::*, px, rgb, Div, FontWeight, SharedString, Styled};

use crate::ui::components::icon;
use crate::ui::theme;

pub(super) fn meta_item(icon_name: &str, text: &str) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(icon(icon_name, 13., theme::FAINT))
        .child(text.to_string())
}

pub(super) fn key_value_row(label: &str, value: impl Into<SharedString>) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
        .py(px(9.))
        .border_b_1()
        .border_color(rgb(theme::BORDER_SOFT))
        .child(
            div()
                .text_size(px(11.5))
                .text_color(rgb(theme::MUTED))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_size(px(11.5))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme::TEXT))
                .text_ellipsis()
                .child(value.into()),
        )
}

pub(super) fn credential_field(label: &str, value: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .min_w(px(120.))
        .child(
            div()
                .text_size(px(9.5))
                .text_color(rgb(theme::FAINT))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(rgb(theme::TEXT))
                .text_ellipsis()
                .child(value.to_string()),
        )
}
