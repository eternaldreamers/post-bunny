use gpui::{
    div, prelude::*, px, rgb, AnyElement, Context, Div, FontWeight, IntoElement, Stateful, Styled,
};

use crate::domain::navigation::Page;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    badge, card, element_id, empty_state, icon, severity_color, severity_icon,
};
use crate::ui::layout;
use crate::ui::theme;

impl Panel {
    pub(crate) fn render_home(&mut self, width: f32, cx: &mut Context<Self>) -> AnyElement {
        let language = self.language;
        let compact = layout::is_compact(width);
        let total_customers = self.customers.len();
        let unread = self.unread_message_count();
        let total_messages = self.total_message_count();
        let open_alerts = self.open_alerts_count();
        let attachments = self.attachment_count();

        let alert_rows = self
            .open_alerts()
            .iter()
            .take(6)
            .map(|alert| {
                let alert_id = alert.id;
                let customer = self
                    .customer(alert.customer_id)
                    .map(|customer| customer.title())
                    .unwrap_or_else(|| {
                        i18n::t(language, "Unknown customer", "Cliente desconocido").into()
                    });
                let ruc = self
                    .customer(alert.customer_id)
                    .map(|customer| customer.ruc.clone())
                    .unwrap_or_default();
                let severity = alert.severity;
                div()
                    .id(element_id(format!("home-alert-{}", alert.id)))
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .cursor_pointer()
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
                            .size(px(30.))
                            .rounded_lg()
                            .bg(theme::tint(severity_color(severity), 0.11))
                            .child(icon(severity_icon(severity), 15., severity_color(severity))),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.))
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
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::MUTED))
                                    .child(format!(
                                        "{customer} · RUC {ruc} · {}",
                                        alert.time_label(language)
                                    )),
                            ),
                    )
                    .child(badge(severity.label(language), severity_color(severity)))
                    .child(icon("chevron-right", 14., theme::FAINT))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let header = div()
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
                            .text_size(px(24.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(theme::TEXT))
                            .child(i18n::t(
                                language,
                                "Operations overview",
                                "Resumen de operaciones",
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(12.5))
                            .text_color(rgb(theme::MUTED))
                            .child(Page::Home.subtitle(language)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .h(px(32.))
                    .px_3()
                    .rounded_lg()
                    .bg(rgb(theme::SURFACE))
                    .border_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .text_size(px(11.5))
                    .text_color(rgb(theme::MUTED))
                    .child(icon("clock", 13., theme::FAINT))
                    .child(self.today.clone()),
            );

        let kpis = div()
            .flex()
            .gap_4()
            .child(stat_card(
                "users",
                i18n::t(language, "SUNAT customers", "Clientes SUNAT"),
                total_customers.to_string(),
                i18n::t(language, "Monitored accounts", "Cuentas monitoreadas").into(),
                theme::ACCENT,
            ))
            .child(stat_card(
                "mail",
                i18n::t(language, "Unread messages", "Mensajes sin leer"),
                unread.to_string(),
                format!(
                    "{total_messages} {}",
                    i18n::t(language, "across all mailboxes", "en todos los buzones")
                ),
                theme::INFO,
            ))
            .child(stat_card(
                "bell",
                i18n::t(language, "Open alerts", "Alertas abiertas"),
                open_alerts.to_string(),
                i18n::t(
                    language,
                    "New SUNAT messages to review",
                    "Nuevos mensajes SUNAT por revisar",
                )
                .into(),
                theme::DANGER,
            ));

        let alerts_card = card()
            .flex_1()
            .min_w(px(420.))
            .child(section_header(
                i18n::t(language, "Open alerts", "Alertas abiertas"),
                Some(i18n::t(
                    language,
                    "New SUNAT messages that need a human.",
                    "Nuevos mensajes SUNAT que requieren atención.",
                )),
                Some(
                    ghost_link(i18n::t(language, "View all", "Ver todo"))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.navigate(Page::Alerts, cx);
                        })),
                ),
            ))
            .child(if alert_rows.is_empty() {
                empty_state(
                    "circle-check",
                    i18n::t(language, "No open alerts", "No hay alertas abiertas"),
                    i18n::t(
                        language,
                        "New SUNAT messages will show up here when a sync finds them.",
                        "Los nuevos mensajes SUNAT aparecerán aquí cuando una sincronización los encuentre.",
                    ),
                )
                .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .children(alert_rows)
                    .into_any_element()
            });

        let mailbox_card = card()
            .flex_1()
            .min_w(px(280.))
            .child(section_header(
                i18n::t(language, "Mailbox", "Buzón"),
                Some(i18n::t(
                    language,
                    "Unread mail and attachment activity.",
                    "Actividad de correo no leído y adjuntos.",
                )),
                None,
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .child(mailbox_row(
                        i18n::t(language, "Unread messages", "Mensajes sin leer"),
                        unread,
                        "mail",
                        theme::INFO,
                    ))
                    .child(mailbox_row(
                        i18n::t(language, "Total messages", "Mensajes totales"),
                        total_messages,
                        "inbox",
                        theme::ACCENT,
                    ))
                    .child(mailbox_row(
                        i18n::t(language, "Attachments available", "Adjuntos disponibles"),
                        attachments,
                        "download",
                        theme::WARNING,
                    )),
            );

        div()
            .id("home-scroll")
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
                    .child(header)
                    .child(kpis)
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_5()
                            .child(alerts_card)
                            .child(mailbox_card),
                    ),
            )
            .into_any_element()
    }
}

fn stat_card(icon_name: &str, label: &str, value: String, note: String, color: u32) -> Div {
    card()
        .flex_1()
        .min_w(px(160.))
        .p_4()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(30.))
                        .rounded_lg()
                        .bg(theme::tint(color, 0.12))
                        .child(icon(icon_name, 15., color)),
                )
                .child(
                    div()
                        .text_size(px(10.5))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme::MUTED))
                        .child(label.to_string()),
                ),
        )
        .child(
            div()
                .text_size(px(26.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme::TEXT))
                .child(value),
        )
        .child(
            div()
                .text_size(px(10.5))
                .text_color(rgb(theme::FAINT))
                .child(note),
        )
}

fn mailbox_row(label: &str, value: usize, icon_name: &str, color: u32) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(26.))
                        .rounded_md()
                        .bg(theme::tint(color, 0.11))
                        .child(icon(icon_name, 13., color)),
                )
                .child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(theme::MUTED))
                        .child(label.to_string()),
                ),
        )
        .child(
            div()
                .text_size(px(12.5))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme::TEXT))
                .child(value.to_string()),
        )
}

fn section_header(title: &str, subtitle: Option<&str>, action: Option<Stateful<Div>>) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
        .p_4()
        .border_b_1()
        .border_color(rgb(theme::BORDER_SOFT))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(13.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::TEXT))
                        .child(title.to_string()),
                )
                .children(subtitle.map(|subtitle| {
                    div()
                        .text_size(px(10.5))
                        .text_color(rgb(theme::MUTED))
                        .child(subtitle.to_string())
                })),
        )
        .children(action)
}

fn ghost_link(label: &str) -> Stateful<Div> {
    div()
        .id(element_id(format!("link-{label}")))
        .flex()
        .items_center()
        .gap_1()
        .px_2()
        .h(px(26.))
        .rounded_md()
        .text_size(px(11.))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgb(theme::ACCENT))
        .cursor_pointer()
        .hover(|style| style.bg(theme::tint(theme::ACCENT, 0.1)))
        .child(label.to_string())
        .child(icon("chevron-right", 12., theme::ACCENT))
}
