use gpui::{div, prelude::*, px, rgb, AnyElement, Context, IntoElement, Styled};

use crate::config;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::layout;
use crate::ui::theme;

impl Panel {
    pub(crate) fn render_statusbar(
        &mut self,
        compact: bool,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let customers = i18n::customers(language, self.customers.len());
        let unread = i18n::unread_badge(language, self.unread_message_count());
        let left_text = if compact {
            format!("{customers} · {unread}")
        } else {
            format!(
                "{customers} · {unread} · {}",
                i18n::open_alerts(language, self.open_alerts_count())
            )
        };

        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .h(px(layout::STATUSBAR_HEIGHT))
            .px(if compact { px(12.) } else { px(20.) })
            .border_t_1()
            .border_color(rgb(theme::BORDER_SOFT))
            .bg(rgb(theme::SIDEBAR))
            .text_size(px(10.))
            .text_color(rgb(theme::FAINT))
            .child(div().flex().items_center().gap_2().child(left_text))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .children((!compact).then(|| {
                        div()
                            .text_color(rgb(theme::FAINT))
                            .child(i18n::shortcuts_hint(language, layout::shortcut_modifier()))
                    }))
                    .child(format!("v{}", config::APP_VERSION)),
            )
            .into_any_element()
    }
}
