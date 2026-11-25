use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::message::SunatMessage;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    card, element_id, ghost_button, icon, icon_button, message_kind_color, message_kind_icon,
    plain_badge,
};
use crate::ui::theme;

impl Panel {
    pub(super) fn message_reading(
        &mut self,
        customer_id: u64,
        message: &SunatMessage,
        show_back: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let cod_mensaje = message.cod_mensaje;
        let kind = message.kind;
        let loading = self.loading_messages.contains(&cod_mensaje);
        let paragraphs = body_paragraphs(&message.body);

        let attachments = message
            .attachments
            .iter()
            .enumerate()
            .map(|(index, attachment)| {
                let attachment_id = attachment.clone();
                let downloadable = attachment.downloadable;
                div()
                    .id(element_id(format!(
                        "attachment-{}-{}",
                        message.cod_mensaje, index
                    )))
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_3()
                    .py_2()
                    .rounded_lg()
                    .bg(rgb(theme::SURFACE_2))
                    .border_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .when(downloadable, |this| {
                        this.cursor_pointer()
                            .hover(|style| style.bg(rgb(theme::SURFACE_3)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.open_attachment_preview(
                                    customer_id,
                                    cod_mensaje,
                                    index,
                                    attachment_id.clone(),
                                    cx,
                                );
                            }))
                    })
                    .child(icon(
                        if downloadable { "eye" } else { "mail" },
                        14.,
                        if downloadable {
                            theme::ACCENT
                        } else {
                            theme::FAINT
                        },
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(theme::TEXT))
                                    .text_ellipsis()
                                    .child(attachment.name.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .text_size(px(9.5))
                                    .text_color(rgb(theme::FAINT))
                                    .child(attachment.size.clone())
                                    .children(attachment.generated.then(|| {
                                        plain_badge(
                                            i18n::t(language, "Generated", "Generado"),
                                            theme::INFO,
                                        )
                                    })),
                            ),
                    )
                    .children(downloadable.then(|| {
                        div()
                            .text_size(px(9.5))
                            .text_color(rgb(theme::ACCENT))
                            .child(i18n::t(language, "Preview", "Vista previa"))
                    }))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        card()
            .flex_1()
            .min_w(px(360.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .p_3()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .children(show_back.then(|| {
                        icon_button("back-to-inbox", "arrow-left")
                            .on_click(cx.listener(Self::close_message))
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .flex_1()
                            .min_w_0()
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .text_size(px(14.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(theme::TEXT))
                                    .text_ellipsis()
                                    .child(message.subject.clone()),
                            )
                            .child(
                                plain_badge(
                                    message.kind.label(language),
                                    message_kind_color(kind),
                                )
                                .flex_none(),
                            ),
                    )
                    .child(
                        ghost_button(
                            "mark-unread",
                            i18n::t(language, "Mark unread", "Marcar como no leído"),
                        )
                        .flex_none()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.mark_message_unread(cod_mensaje, cx);
                        })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .p_4()
                    .border_b_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .size(px(36.))
                            .rounded_full()
                            .bg(theme::tint(message_kind_color(kind), 0.12))
                            .child(icon(
                                message_kind_icon(kind),
                                16.,
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
                                    .text_size(px(12.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(theme::TEXT))
                                    .child("SUNAT"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .flex_wrap()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::FAINT))
                                    .child(format!(
                                        "{} · {} {}",
                                        message.fec_publica,
                                        i18n::t(language, "code", "código"),
                                        message.cod_mensaje
                                    ))
                                    .child(format!(
                                        "· {} {}",
                                        i18n::t(language, "sent", "enviado"),
                                        message.fec_envio
                                    ))
                                    .children(message.vigencia.as_ref().map(|vigencia| {
                                        div().child(format!(
                                            "· {} {vigencia}",
                                            i18n::t(language, "valid until", "válido hasta")
                                        ))
                                    }))
                                    .children((message.attachment_count > 0).then(|| {
                                        format!(
                                            "· {}",
                                            i18n::attachments(language, message.attachment_count)
                                        )
                                    })),
                            ),
                    )
                    .children(
                        message
                            .label
                            .as_ref()
                            .map(|label| plain_badge(&label.name, label.color)),
                    ),
            )
            .child(if message.body_loaded {
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_5()
                    .children(paragraphs)
                    .into_any_element()
            } else if loading {
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .p_5()
                    .child(icon("refresh-cw", 16., theme::ACCENT))
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(theme::MUTED))
                            .child(i18n::t(
                                language,
                                "Loading message from SUNAT…",
                                "Cargando mensaje desde SUNAT…",
                            )),
                    )
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_5()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(theme::MUTED))
                            .child(i18n::t(
                                language,
                                "Message body not loaded yet.",
                                "El cuerpo del mensaje aún no está cargado.",
                            )),
                    )
                    .child(
                        ghost_button(
                            "load-message",
                            i18n::t(language, "Load from SUNAT", "Cargar desde SUNAT"),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.load_message_body(customer_id, cod_mensaje, cx);
                        })),
                    )
                    .children((message.attachment_count > 0).then(|| {
                        div()
                            .text_size(px(10.5))
                            .text_color(rgb(theme::FAINT))
                            .child(format!(
                                "{} {}",
                                i18n::attachments(language, message.attachment_count),
                                i18n::t(
                                    language,
                                    "will load with the message.",
                                    "se cargarán con el mensaje.",
                                ),
                            ))
                    }))
                    .into_any_element()
            })
            .child(if attachments.is_empty() {
                div().into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .px_5()
                    .pb_5()
                    .child(
                        div()
                            .text_size(px(9.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(theme::FAINT))
                            .child(i18n::t(language, "ATTACHMENTS", "ADJUNTOS")),
                    )
                    .children(attachments)
                    .child(
                        div()
                            .text_size(px(9.5))
                            .text_color(rgb(theme::FAINT))
                            .child(i18n::t(
                                language,
                                "Attachment names come from the SUNAT viewer; download them from the portal.",
                                "Los nombres de los adjuntos provienen del visor SUNAT; descárgalos desde el portal.",
                            )),
                    )
                    .into_any_element()
            })
            .into_any_element()
    }
}

fn body_paragraphs(body: &str) -> Vec<AnyElement> {
    body.split('\n')
        .map(|line| {
            div()
                .text_size(px(12.5))
                .line_height(px(21.))
                .text_color(rgb(if line.is_empty() {
                    theme::BORDER_SOFT
                } else {
                    theme::MUTED
                }))
                .child(if line.is_empty() {
                    " ".to_string()
                } else {
                    line.to_string()
                })
                .into_any_element()
        })
        .collect()
}
