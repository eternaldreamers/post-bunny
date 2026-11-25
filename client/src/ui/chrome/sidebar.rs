use gpui::{
    div, prelude::*, px, rgb, rgba, AnyElement, Context, FontWeight, IntoElement, Stateful, Styled,
};

use crate::domain::navigation::Page;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{element_id, icon, plain_badge};
use crate::ui::layout::{self, SidebarMode};
use crate::ui::theme;

impl Panel {
    pub(crate) fn render_sidebar(
        &mut self,
        mode: SidebarMode,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match mode {
            SidebarMode::Expanded => self.sidebar_inner(layout::SIDEBAR_EXPANDED_WIDTH, true, cx),
            SidebarMode::Rail => self.sidebar_inner(layout::SIDEBAR_RAIL_WIDTH, false, cx),
            SidebarMode::Hidden => div().into_any_element(),
        }
    }

    pub(crate) fn render_drawer(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let sidebar = self.sidebar_inner(layout::SIDEBAR_DRAWER_WIDTH, true, cx);
        div()
            .absolute()
            .top_0()
            .left_0()
            .flex()
            .h_full()
            .w_full()
            .child(div().flex_none().h_full().shadow_lg().child(sidebar))
            .child(
                div()
                    .id("drawer-backdrop")
                    .flex_1()
                    .h_full()
                    .bg(rgba(theme::OVERLAY_DRAWER))
                    .occlude()
                    .on_click(cx.listener(Self::close_drawer)),
            )
            .into_any_element()
    }

    fn sidebar_inner(&self, width: f32, expanded: bool, cx: &mut Context<Self>) -> AnyElement {
        let open_alerts = self.open_alerts_count();
        let customers = self.customers.len();

        let brand = div()
            .flex()
            .items_center()
            .gap_3()
            .px(if expanded { px(10.) } else { px(0.) })
            .py_2()
            .when(!expanded, |this| this.justify_center())
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .size(px(32.))
                    .rounded_lg()
                    .bg(theme::tint(theme::ACCENT, 0.16))
                    .border_1()
                    .border_color(theme::tint(theme::ACCENT, 0.32))
                    .child(icon("rabbit", 17., theme::ACCENT)),
            )
            .children(expanded.then(|| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(1.))
                    .child(
                        div()
                            .text_size(px(13.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(theme::TEXT))
                            .child(crate::config::APP_NAME),
                    )
                    .child(
                        div()
                            .text_size(px(9.5))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(theme::FAINT))
                            .child(i18n::t(
                                self.language,
                                "CUSTOMER OPERATIONS",
                                "OPERACIONES DE CLIENTES",
                            )),
                    )
            }));

        let nav = div()
            .flex()
            .flex_col()
            .gap_1()
            .when(expanded, |this| this.mt_6())
            .child(
                self.nav_item(Page::Home, expanded, None)
                    .on_click(cx.listener(Self::nav_home)),
            )
            .child(
                self.nav_item(Page::Customers, expanded, Some((customers, theme::MUTED)))
                    .on_click(cx.listener(Self::nav_customers)),
            )
            .child(
                self.nav_item(Page::Alerts, expanded, Some((open_alerts, theme::DANGER)))
                    .on_click(cx.listener(Self::nav_alerts)),
            );

        div()
            .flex()
            .flex_none()
            .flex_col()
            .w(px(width))
            .h_full()
            .bg(rgb(theme::SIDEBAR))
            .border_r_1()
            .border_color(rgb(theme::BORDER_SOFT))
            .px(if expanded { px(12.) } else { px(10.) })
            .py_4()
            .child(div().flex().flex_col().gap_1().child(brand).child(nav))
            .into_any_element()
    }

    fn nav_item(
        &self,
        page: Page,
        expanded: bool,
        count: Option<(usize, u32)>,
    ) -> Stateful<gpui::Div> {
        let active = self.page == page;
        let icon_name = match page {
            Page::Home => "home",
            Page::Customers => "users",
            Page::Alerts => "bell",
        };
        div()
            .id(element_id(format!("nav-{}", page_slug(page))))
            .flex()
            .items_center()
            .gap_3()
            .h(px(38.))
            .px(px(10.))
            .rounded_lg()
            .cursor_pointer()
            .when(active, |this| {
                this.bg(theme::tint(theme::ACCENT, 0.12))
                    .text_color(rgb(theme::TEXT))
            })
            .when(!active, |this| {
                this.text_color(rgb(theme::MUTED))
                    .hover(|style| style.bg(rgb(theme::SURFACE_2)).text_color(rgb(theme::TEXT)))
            })
            .child(icon(
                icon_name,
                16.,
                if active { theme::ACCENT } else { theme::MUTED },
            ))
            .child(
                div()
                    .flex_1()
                    .when(expanded, |this| this.child(page.title(self.language))),
            )
            .children(match count {
                Some((count, color)) if expanded && count > 0 => {
                    Some(plain_badge(&count.to_string(), color))
                }
                _ => None,
            })
    }
}

fn page_slug(page: Page) -> &'static str {
    match page {
        Page::Home => "home",
        Page::Customers => "customers",
        Page::Alerts => "alerts",
    }
}
