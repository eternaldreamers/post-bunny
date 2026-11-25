use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::i18n::{self, Language};
use crate::panel::Panel;
use crate::ui::components::{element_id, icon};
use crate::ui::layout;
use crate::ui::theme;

impl Panel {
    pub(super) fn render_language_menu(
        &mut self,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let item = |target: Language, active: bool, cx: &mut Context<Self>| {
            let mut row = div()
                .id(element_id(format!("language-{}", target.label())))
                .flex()
                .items_center()
                .justify_between()
                .h(px(34.))
                .px_3()
                .rounded_md()
                .cursor_pointer()
                .text_size(px(12.))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme::TEXT))
                .when(active, |this| this.bg(theme::tint(theme::ACCENT, 0.13)))
                .when(!active, |this| {
                    this.hover(|style| style.bg(rgb(theme::SURFACE_3)))
                })
                .child(target.label());
            if active {
                row = row.child(icon("check", 14., theme::ACCENT));
            }
            row.on_click(cx.listener(move |this, _, window, cx| {
                this.set_language(target, window, cx);
            }))
        };

        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .child(
                div()
                    .id("language-menu-backdrop")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .on_click(cx.listener(Self::close_language_menu)),
            )
            .child(
                div()
                    .absolute()
                    .top(px(58.))
                    .right(px(if layout::is_compact(width) { 16. } else { 24. }))
                    .w(px(190.))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_2()
                    .bg(rgb(theme::SURFACE_2))
                    .border_1()
                    .border_color(rgb(theme::BORDER))
                    .rounded_lg()
                    .shadow_lg()
                    .child(
                        div()
                            .px_2()
                            .pt_1()
                            .pb_2()
                            .text_size(px(9.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(theme::FAINT))
                            .child(i18n::t(language, "LANGUAGE", "IDIOMA")),
                    )
                    .children(
                        Language::ALL
                            .into_iter()
                            .map(|target| item(target, language == target, cx)),
                    ),
            )
            .into_any_element()
    }
}
