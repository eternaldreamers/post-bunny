use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::customer::Customer;
use crate::domain::message::SunatMessage;
use crate::i18n;
use crate::i18n::Language;
use crate::panel::Panel;
use crate::ui::components::{
    card, element_id, empty_state, ghost_button, icon, message_kind_color, message_kind_icon,
    plain_badge,
};
use crate::ui::layout;
use crate::ui::theme;

impl Panel {
    pub(super) fn customer_inbox(
        &mut self,
        customer: &Customer,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let customer_id = customer.id;
        let wide = layout::is_inbox_wide(width);
        let messages = customer.messages.clone();
        let open_message = self
            .reading_message_id
            .and_then(|id| messages.iter().find(|message| message.cod_mensaje == id))
            .cloned();
        let unread = customer.unread_count();

        let rows = messages
            .iter()
            .map(|message| message_row(message, language, cx))
            .collect::<Vec<_>>();

        let list = card()
            .flex_1()
            .min_w(px(320.))
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
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(i18n::t(language, "Inbox", "Bandeja")),
                            )
                            .child(plain_badge(
                                &i18n::unread_badge(language, unread),
                                if unread > 0 {
                                    theme::ACCENT
                                } else {
                                    theme::FAINT
                                },
                            )),
                    )
                    .children((unread > 0).then(|| {
                        ghost_button(
                            "mark-all-read",
                            i18n::t(language, "Mark all read", "Marcar todo leído"),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.mark_all_messages_read(customer_id, cx);
                        }))
                    })),
            )
            .child(if rows.is_empty() {
                empty_state(
                    "inbox",
                    i18n::t(language, "Mailbox not synced", "Buzón sin sincronizar"),
                    i18n::t(
                        language,
                        "SUNAT messages will appear here after the first sync for this customer.",
                        "Los mensajes SUNAT aparecerán aquí después de la primera sincronización de este cliente.",
                    ),
                )
                .into_any_element()
            } else {
                div().flex().flex_col().children(rows).into_any_element()
            });

        if let Some(message) = open_message {
            let reading = self.message_reading(customer_id, &message, !wide, cx);
            if wide {
                div()
                    .flex()
                    .gap_5()
                    .items_start()
                    .child(list)
                    .child(reading)
                    .into_any_element()
            } else {
                reading.into_any_element()
            }
        } else if wide {
            div()
                .flex()
                .gap_5()
                .items_start()
                .child(list)
                .child(
                    card().flex_1().min_w(px(360.)).child(
                        empty_state(
                            "mail",
                            i18n::t(language, "Select a message", "Selecciona un mensaje"),
                            i18n::t(
                                language,
                                "Choose a SUNAT message from the list to read it here.",
                                "Elige un mensaje SUNAT de la lista para leerlo aquí.",
                            ),
                        )
                        .into_any_element(),
                    ),
                )
                .into_any_element()
        } else {
            list.into_any_element()
        }
    }
}

fn message_row(message: &SunatMessage, language: Language, cx: &mut Context<Panel>) -> AnyElement {
    let cod_mensaje = message.cod_mensaje;
    let kind = message.kind;
    let unread = !message.read;
    let label = message.label.clone();

    div()
        .id(element_id(format!("inbox-message-{}", message.cod_mensaje)))
        .flex()
        .items_center()
        .gap_3()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(rgb(theme::BORDER_SOFT))
        .cursor_pointer()
        .when(unread, |this| this.bg(theme::tint(theme::ACCENT, 0.035)))
        .hover(|style| style.bg(rgb(theme::SURFACE_2)))
        .on_click(cx.listener(move |this, _, _, cx| {
            this.open_message(cod_mensaje, cx);
        }))
        .child(if unread {
            div()
                .flex_none()
                .size(px(6.))
                .rounded_full()
                .bg(rgb(theme::ACCENT))
                .into_any_element()
        } else {
            div().flex_none().size(px(6.)).into_any_element()
        })
        .child(
            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .size(px(28.))
                .rounded_md()
                .bg(theme::tint(message_kind_color(kind), 0.1))
                .child(icon(message_kind_icon(kind), 13., message_kind_color(kind))),
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
                        .justify_between()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .text_size(px(11.5))
                                .font_weight(if unread {
                                    FontWeight::SEMIBOLD
                                } else {
                                    FontWeight::MEDIUM
                                })
                                .text_color(rgb(theme::TEXT))
                                .text_ellipsis()
                                .child(message.subject.clone()),
                        )
                        .child(
                            div()
                                .flex_none()
                                .text_size(px(9.5))
                                .text_color(rgb(theme::FAINT))
                                .child(message.fec_publica.clone()),
                        ),
                )
                .child(
                    div()
                        .text_size(px(10.5))
                        .text_color(rgb(theme::FAINT))
                        .text_ellipsis()
                        .child(message.preview()),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_size(px(9.5))
                                .text_color(rgb(theme::MUTED))
                                .child(format!(
                                    "SUNAT · {} · {} {}",
                                    message.kind.label(language),
                                    i18n::t(language, "code", "código"),
                                    message.cod_mensaje
                                )),
                        )
                        .children((!message.attachments.is_empty()).then(|| {
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .text_size(px(9.5))
                                .text_color(rgb(theme::MUTED))
                                .child(icon("mail", 10., theme::MUTED))
                                .child(message.attachment_count.to_string())
                        }))
                        .children(message.urgent.then(|| {
                            plain_badge(i18n::t(language, "Urgent", "Urgente"), theme::DANGER)
                        }))
                        .children(label.map(|label| plain_badge(&label.name, label.color))),
                ),
        )
        .child(icon("chevron-right", 14., theme::FAINT))
        .into_any_element()
}
