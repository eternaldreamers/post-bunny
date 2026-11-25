use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::alert::{AlertFilter, AlertState};
use crate::domain::navigation::Page;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    card, chip, element_id, empty_state, icon, icon_button, plain_badge, severity_color,
    severity_icon,
};
use crate::ui::layout;
use crate::ui::theme;

impl Panel {
    pub(crate) fn render_alerts(&mut self, width: f32, cx: &mut Context<Self>) -> AnyElement {
        let language = self.language;
        let compact = layout::is_compact(width);
        let alerts = self.filtered_alerts();
        let total = self.alerts.len();
        let open = self.open_alerts_count();
        let resolved = total.saturating_sub(open);

        let filters = div()
            .flex()
            .flex_wrap()
            .gap_2()
            .children(AlertFilter::ALL.into_iter().map(|filter| {
                let active = self.alert_filter == filter;
                let count = match filter {
                    AlertFilter::All => total,
                    AlertFilter::Open => open,
                    AlertFilter::Resolved => resolved,
                };
                let mut chip = chip(
                    element_id(format!("alert-filter-{}", filter.label(language))),
                    &format!("{}  {}", filter.label(language), count),
                    active,
                );
                if !active {
                    chip = chip.on_click(cx.listener(move |this, _, _, cx| {
                        this.alert_filter = filter;
                        cx.notify();
                    }));
                }
                chip
            }));

        let rows = alerts
            .iter()
            .map(|alert| {
                let alert_id = alert.id;
                let customer = self.customer(alert.customer_id).cloned();
                let customer_name = customer
                    .as_ref()
                    .map(|customer| customer.title())
                    .unwrap_or_else(|| "—".into());
                let ruc = customer
                    .as_ref()
                    .map(|customer| customer.ruc.clone())
                    .unwrap_or_default();
                let severity = alert.severity;
                let resolved = alert.state == AlertState::Resolved;

                div()
                    .id(element_id(format!("alert-row-{}", alert.id)))
                    .flex()
                    .items_center()
                    .gap_4()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .cursor_pointer()
                    .when(resolved, |this| this.opacity(0.55))
                    .hover(|style| style.bg(rgb(theme::SURFACE_2)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.open_alert(alert_id, cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .size(px(32.))
                            .rounded_lg()
                            .bg(theme::tint(severity_color(severity), 0.11))
                            .child(icon(severity_icon(severity), 15., severity_color(severity))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(theme::TEXT))
                                    .text_ellipsis()
                                    .child(alert.title.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .flex_wrap()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::MUTED))
                                    .child(customer_name)
                                    .child(format!("· RUC {ruc}"))
                                    .child(format!("· {}", alert.time_label(language)))
                                    .child(format!("· {}", alert.source))
                                    .child(plain_badge(
                                        severity.label(language),
                                        severity_color(severity),
                                    )),
                            ),
                    )
                    .child(if resolved {
                        plain_badge(i18n::t(language, "Resolved", "Resuelta"), theme::SUCCESS)
                            .into_any_element()
                    } else {
                        icon_button(
                            element_id(format!("resolve-alert-btn-{}", alert.id)),
                            "check",
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.resolve_alert(alert_id, cx);
                            cx.stop_propagation();
                        }))
                        .into_any_element()
                    })
                    .child(icon("chevron-right", 14., theme::FAINT))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let list = card().child(if rows.is_empty() {
            empty_state(
                "bell",
                i18n::t(
                    language,
                    "No alerts in this view",
                    "No hay alertas en esta vista",
                ),
                i18n::t(
                    language,
                    "New SUNAT messages that need attention will show up here.",
                    "Los nuevos mensajes SUNAT que requieran atención aparecerán aquí.",
                ),
            )
            .into_any_element()
        } else {
            div().flex().flex_col().children(rows).into_any_element()
        });

        div()
            .id("alerts-scroll")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .p(px(layout::content_padding(compact)))
                    .pb(px(32.))
                    .w_full()
                    .max_w(px(layout::CONTENT_MAX_WIDTH))
                    .mx_auto()
                    .child(
                        div()
                            .flex()
                            .items_end()
                            .justify_between()
                            .gap_4()
                            .flex_wrap()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_size(px(24.))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .child(Page::Alerts.title(language)),
                                            )
                                            .child(plain_badge(
                                                &i18n::open_badge(language, open),
                                                if open > 0 {
                                                    theme::WARNING
                                                } else {
                                                    theme::SUCCESS
                                                },
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.5))
                                            .text_color(rgb(theme::MUTED))
                                            .child(i18n::t(
                                                language,
                                                "New SUNAT messages that need attention. Open one to jump to its inbox.",
                                                "Nuevos mensajes SUNAT que requieren atención. Abre uno para ir a su bandeja.",
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::FAINT))
                                    .child(i18n::showing_alerts(language, alerts.len(), total)),
                            ),
                    )
                    .child(filters)
                    .child(list),
            )
            .into_any_element()
    }
}
