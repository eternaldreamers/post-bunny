use gpui::{
    div, prelude::*, px, relative, rgb, rgba, AnyElement, App, ClickEvent, Div, FontWeight,
    IntoElement, Styled, Window,
};

use crate::ui::components::icon_button;
use crate::ui::theme;

pub(super) fn modal_overlay(card: AnyElement) -> AnyElement {
    div()
        .absolute()
        .top_0()
        .left_0()
        .flex()
        .items_center()
        .justify_center()
        .size_full()
        .occlude()
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .bg(rgba(theme::OVERLAY_BACKDROP)),
        )
        .child(card)
        .into_any_element()
}

pub(super) fn modal_card(width: f32, min_width: f32, max_width: f32) -> Div {
    div()
        .flex()
        .flex_col()
        .w(px((width - 32.).clamp(min_width, max_width)))
        .max_h(relative(0.9))
        .bg(rgb(theme::SURFACE))
        .border_1()
        .border_color(rgb(theme::BORDER))
        .rounded_xl()
        .shadow_lg()
        .overflow_hidden()
}

pub(super) fn modal_header(
    title: &str,
    subtitle: &str,
    close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Div {
    div()
        .flex()
        .items_start()
        .justify_between()
        .gap_4()
        .p_5()
        .border_b_1()
        .border_color(rgb(theme::BORDER_SOFT))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(16.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::TEXT))
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(theme::MUTED))
                        .child(subtitle.to_string()),
                ),
        )
        .child(icon_button("modal-close", "x").flex_none().on_click(close))
}
