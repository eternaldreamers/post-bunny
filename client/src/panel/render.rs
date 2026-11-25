use gpui::{prelude::*, px, Context, IntoElement, Render, Window};

use crate::config;
use crate::domain::navigation::Page;
use crate::ui::{layout, theme};

use super::Panel;

impl Render for Panel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let width: f32 = window.viewport_size().width.into();
        let mode = layout::sidebar_mode(width);
        let compact = layout::is_compact(width);

        let content = match self.page {
            Page::Home => self.render_home(width, cx),
            Page::Customers => self.render_customers(width, cx),
            Page::Alerts => self.render_alerts(width, cx),
        };

        let mut overlays = self.render_overlays(width, cx);
        if matches!(mode, layout::SidebarMode::Hidden) && self.drawer_open {
            overlays.insert(0, self.render_drawer(cx));
        }

        gpui::div()
            .relative()
            .flex()
            .size_full()
            .bg(gpui::rgb(theme::BG))
            .text_color(gpui::rgb(theme::TEXT))
            .font_family(config::FONT_FAMILY)
            .text_size(px(config::BASE_FONT_SIZE))
            .track_focus(&self.root_focus)
            .on_key_down(cx.listener(Self::on_root_key_down))
            .child(self.render_sidebar(mode, cx))
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(self.render_topbar(mode, compact, cx))
                    .child(content)
                    .child(self.render_statusbar(compact, cx)),
            )
            .children(overlays)
    }
}
