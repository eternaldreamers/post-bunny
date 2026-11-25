use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::navigation::Page;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{icon, icon_button};
use crate::ui::layout::{self, SidebarMode};
use crate::ui::theme;

impl Panel {
    pub(crate) fn render_topbar(
        &mut self,
        mode: SidebarMode,
        compact: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let selected = (self.page == Page::Customers)
            .then(|| self.selected_customer().cloned())
            .flatten();

        let left = div()
            .flex()
            .items_center()
            .gap_3()
            .min_w_0()
            .when(mode == SidebarMode::Hidden, |this| {
                this.child(
                    icon_button("open-nav", "menu").on_click(cx.listener(Self::toggle_drawer)),
                )
            })
            .when_some(selected, |this, customer| {
                this.child(
                    icon_button("back-to-customers", "arrow-left")
                        .on_click(cx.listener(Self::back_to_customers)),
                )
                .child(
                    div()
                        .id("breadcrumb-customers")
                        .text_size(px(12.))
                        .text_color(rgb(theme::MUTED))
                        .cursor_pointer()
                        .hover(|style| style.text_color(rgb(theme::TEXT)))
                        .on_click(cx.listener(Self::go_customers_list))
                        .child(i18n::t(language, "Customers", "Clientes")),
                )
                .child(icon("chevron-right", 13., theme::FAINT))
                .child(
                    div()
                        .text_size(px(13.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme::TEXT))
                        .text_ellipsis()
                        .child(customer.title()),
                )
            })
            .when(
                self.selected_customer().is_none() || self.page != Page::Customers,
                |this| {
                    this.when(!compact, |this| {
                        this.child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(theme::FAINT))
                                .child(i18n::t(language, "Workspace", "Espacio de trabajo")),
                        )
                        .child(icon("chevron-right", 13., theme::FAINT))
                    })
                    .child(
                        div()
                            .text_size(px(13.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(theme::TEXT))
                            .child(self.page.title(language)),
                    )
                },
            );

        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .gap_4()
            .h(px(layout::TOPBAR_HEIGHT))
            .px(if compact { px(16.) } else { px(24.) })
            .border_b_1()
            .border_color(rgb(theme::BORDER_SOFT))
            .bg(rgb(theme::BG))
            .child(left)
            .child(
                div().flex().items_center().child(
                    div()
                        .id("language-menu-button")
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(34.))
                        .rounded_lg()
                        .cursor_pointer()
                        .hover(|style| style.bg(rgb(theme::SURFACE_2)))
                        .on_click(cx.listener(Self::toggle_language_menu))
                        .child(icon("languages", 17., theme::MUTED)),
                ),
            )
            .into_any_element()
    }
}
