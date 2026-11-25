use gpui::{div, prelude::*, px, rgb, AnyElement, Context, IntoElement, Styled};

use crate::panel::{Panel, ToastKind};
use crate::ui::components::icon;
use crate::ui::theme;

impl Panel {
    pub(super) fn render_toast(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let Some(toast) = self.toast.clone() else {
            return div().into_any_element();
        };
        let (color, icon_name) = match toast.kind {
            ToastKind::Success => (theme::SUCCESS, "circle-check"),
            ToastKind::Info => (theme::INFO, "info"),
            ToastKind::Danger => (theme::DANGER, "alert-circle"),
        };

        div()
            .absolute()
            .bottom_5()
            .right_5()
            .flex()
            .items_center()
            .gap_3()
            .min_w(px(260.))
            .max_w(px(360.))
            .px_4()
            .py_3()
            .rounded_xl()
            .bg(rgb(theme::SURFACE_3))
            .border_1()
            .border_color(theme::tint(color, 0.3))
            .shadow_lg()
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .size(px(26.))
                    .rounded_full()
                    .bg(theme::tint(color, 0.14))
                    .child(icon(icon_name, 14., color)),
            )
            .child(
                div()
                    .flex_1()
                    .text_size(px(12.))
                    .text_color(rgb(theme::TEXT))
                    .child(toast.message),
            )
            .child(
                div()
                    .id("dismiss-toast")
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(22.))
                    .rounded_md()
                    .cursor_pointer()
                    .text_color(rgb(theme::FAINT))
                    .hover(|style| style.bg(theme::tint(theme::TEXT, 0.08)))
                    .on_click(cx.listener(Self::dismiss_toast))
                    .child(icon("x", 12., theme::FAINT)),
            )
            .into_any_element()
    }
}
