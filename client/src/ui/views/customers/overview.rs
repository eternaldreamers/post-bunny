use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::alert::Alert;
use crate::domain::customer::Customer;
use crate::domain::navigation::{DetailTab, Page};
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    card, element_id, empty_state, ghost_button, icon, message_kind_color, message_kind_icon,
    severity_color, severity_icon,
};
use crate::ui::theme;

use super::widgets::key_value_row;

impl Panel {
    pub(super) fn customer_overview(
        &mut self,
        customer: &Customer,
        alerts: &[Alert],
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let message_count = customer.message_count();
        let unread = customer.unread_count();
        let notifications = customer.notification_count();
        let attachments = customer.attachment_count();
        let recent_messages = customer
            .messages
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>();
        let recent_alerts = alerts.iter().take(3).cloned().collect::<Vec<_>>();

        let detail_rows = card()
            .min_w(px(340.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .p_4()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .child(
                        div()
                            .text_size(px(13.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(i18n::t(language, "SUNAT account", "Cuenta SUNAT")),
                    )
                    .child(icon("shield", 15., theme::FAINT)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .px_4()
                    .py_1()
                    .child(key_value_row(
                        i18n::t(language, "Login type", "Tipo de acceso"),
                        customer.login_type.label(),
                    ))
                    .child(key_value_row(
                        "DNI",
                        customer.dni.clone().unwrap_or_else(|| "—".into()),
                    ))
                    .child(key_value_row("RUC", customer.ruc.clone()))
                    .child(key_value_row(
                        i18n::t(language, "SOL user", "Usuario SOL"),
                        customer.sol_user.clone().unwrap_or_else(|| "—".into()),
                    ))
                    .child(key_value_row(
                        i18n::t(language, "Last sync", "Última sincronización"),
                        customer
                            .last_sync
                            .clone()
                            .unwrap_or_else(|| i18n::t(language, "Never", "Nunca").into()),
                    ))
                    .child(key_value_row(
                        i18n::t(language, "Account ID", "ID de cuenta"),
                        customer.account_label(),
                    )),
            );

        let notes = card().min_w(px(340.)).child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .p_4()
                .child(
                    div()
                        .text_size(px(13.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(i18n::t(language, "Account notes", "Notas de la cuenta")),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(theme::MUTED))
                        .line_height(px(19.))
                        .child(customer.notes.clone()),
                ),
        );

        let mailbox = card()
            .min_w(px(300.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .child(
                        div()
                            .text_size(px(13.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(i18n::t(language, "Mailbox", "Buzón")),
                    )
                    .child(icon("inbox", 15., theme::FAINT)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .child(
                        div()
                            .flex()
                            .items_end()
                            .gap_2()
                            .child(
                                div()
                                    .text_size(px(36.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(if unread > 0 {
                                        theme::ACCENT
                                    } else {
                                        theme::TEXT
                                    }))
                                    .child(unread.to_string()),
                            )
                            .child(
                                div()
                                    .pb(px(6.))
                                    .text_size(px(11.))
                                    .text_color(rgb(theme::FAINT))
                                    .child(i18n::t(language, "unread", "sin leer")),
                            ),
                    )
                    .child(stat_row(
                        i18n::t(language, "Total messages", "Mensajes totales"),
                        message_count,
                    ))
                    .child(stat_row(
                        i18n::t(language, "Notifications", "Notificaciones"),
                        notifications,
                    ))
                    .child(stat_row(
                        i18n::t(language, "Attachments", "Adjuntos"),
                        attachments,
                    )),
            )
            .child(
                div().p_3().child(
                    ghost_button(
                        "open-inbox",
                        i18n::t(language, "Open full inbox", "Abrir bandeja completa"),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.select_detail_tab(DetailTab::Inbox, cx);
                    })),
                ),
            );

        let messages_card = card()
            .min_w(px(330.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .child(
                        div()
                            .text_size(px(13.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(i18n::t(language, "Recent messages", "Mensajes recientes")),
                    )
                    .child(
                        ghost_button(
                            "all-messages",
                            i18n::t(language, "Open inbox", "Abrir bandeja"),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.select_detail_tab(DetailTab::Inbox, cx);
                        })),
                    ),
            )
            .child(if recent_messages.is_empty() {
                empty_state(
                    "inbox",
                    i18n::t(language, "Mailbox not synced", "Buzón sin sincronizar"),
                    i18n::t(
                        language,
                        "Run the first SUNAT sync to load messages for this customer.",
                        "Ejecuta la primera sincronización SUNAT para cargar los mensajes de este cliente.",
                    ),
                )
                .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .children(recent_messages.iter().map(|message| {
                        let cod_mensaje = message.cod_mensaje;
                        let kind = message.kind;
                        div()
                            .id(element_id(format!(
                                "overview-message-{}",
                                message.cod_mensaje
                            )))
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
                                this.open_message(cod_mensaje, cx);
                                this.detail_tab = DetailTab::Inbox;
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .size(px(28.))
                                    .rounded_md()
                                    .bg(theme::tint(message_kind_color(kind), 0.1))
                                    .child(icon(
                                        message_kind_icon(kind),
                                        13.,
                                        message_kind_color(kind),
                                    )),
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
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_size(px(11.5))
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(theme::TEXT))
                                                    .text_ellipsis()
                                                    .child(message.subject.clone()),
                                            )
                                            .children((!message.read).then(|| {
                                                div()
                                                    .flex_none()
                                                    .size(px(5.))
                                                    .rounded_full()
                                                    .bg(rgb(theme::ACCENT))
                                            })),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(9.5))
                                            .text_color(rgb(theme::FAINT))
                                            .child(format!(
                                                "{} · {}",
                                                message.kind.label(language),
                                                message.fec_publica
                                            )),
                                    ),
                            )
                            .into_any_element()
                    }))
                    .into_any_element()
            });

        let alerts_card = card().min_w(px(340.)).child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .p_4()
                        .border_b_1()
                        .border_color(rgb(theme::BORDER_SOFT))
                        .child(
                            div()
                                .text_size(px(13.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(i18n::t(language, "Recent alerts", "Alertas recientes")),
                        )
                        .child(
                            ghost_button(
                                "view-all-alerts",
                                i18n::t(language, "View all", "Ver todo"),
                            )
                            .on_click(
                                cx.listener(|this, _, _, cx| this.navigate(Page::Alerts, cx)),
                            ),
                        ),
                )
                .child(if recent_alerts.is_empty() {
                    div()
                        .p_4()
                        .text_size(px(11.5))
                        .text_color(rgb(theme::FAINT))
                        .child(i18n::t(
                            language,
                            "No alerts for this customer.",
                            "No hay alertas para este cliente.",
                        ))
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .children(recent_alerts.iter().map(|alert| {
                            let severity = alert.severity;
                            div()
                                .id(element_id(format!("overview-alert-{}", alert.id)))
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_4()
                                .py_3()
                                .border_b_1()
                                .border_color(rgb(theme::BORDER_SOFT))
                                .child(
                                    div()
                                        .flex()
                                        .flex_none()
                                        .items_center()
                                        .justify_center()
                                        .size(px(26.))
                                        .rounded_md()
                                        .bg(theme::tint(severity_color(severity), 0.1))
                                        .child(icon(
                                            severity_icon(severity),
                                            13.,
                                            severity_color(severity),
                                        )),
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
                                                .text_size(px(11.5))
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(theme::TEXT))
                                                .text_ellipsis()
                                                .child(alert.title.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(9.5))
                                                .text_color(rgb(theme::FAINT))
                                                .child(format!(
                                                    "{} · {}",
                                                    alert.time_label(language),
                                                    alert.state.label(language)
                                                )),
                                        ),
                                )
                                .into_any_element()
                        }))
                        .into_any_element()
                }),
        );

        div()
            .flex()
            .flex_wrap()
            .gap_5()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .flex_1()
                    .min_w(px(340.))
                    .child(detail_rows)
                    .child(notes),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_5()
                    .flex_1()
                    .min_w(px(300.))
                    .child(mailbox)
                    .child(messages_card)
                    .child(alerts_card),
            )
            .into_any_element()
    }
}

fn stat_row(label: &str, value: usize) -> gpui::Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(px(11.))
                .text_color(rgb(theme::MUTED))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_size(px(12.))
                .font_weight(FontWeight::MEDIUM)
                .child(value.to_string()),
        )
}
