use gpui::{div, prelude::*, px, rgb, Div, Entity, FontWeight};

use crate::ui::text_input::TextInput;
use crate::ui::theme;

pub fn labeled_input(label: &str, input: Entity<TextInput>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .min_w_0()
        .child(field_label(label))
        .child(input)
}

pub fn field_label(label: &str) -> Div {
    div()
        .text_size(px(11.))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgb(theme::MUTED))
        .child(label.to_string())
}
