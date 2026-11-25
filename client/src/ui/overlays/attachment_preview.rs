use gpui::{
    div, img, prelude::*, px, relative, rgb, AnyElement, Context, FontWeight, IntoElement,
    ObjectFit, Styled,
};

use crate::i18n;
use crate::panel::{AttachmentPreviewState, Panel};
use crate::preview::PreviewContent;
use crate::ui::components::{icon, icon_button};
use crate::ui::theme;

use super::modal::modal_overlay;

impl Panel {
    pub(super) fn render_attachment_preview(
        &mut self,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let Some(preview) = self.attachment_preview.as_ref() else {
            return div().into_any_element();
        };
        let name = preview.name.clone();
        let size = preview.size.clone();
        let customer = self
            .customer(preview.customer_id)
            .map(|customer| customer.title())
            .unwrap_or_else(|| i18n::t(language, "Customer", "Cliente").into());
        let ready = matches!(&preview.state, AttachmentPreviewState::Ready { .. });

        let body = match &preview.state {
            AttachmentPreviewState::Loading => centered_message(
                "refresh-cw",
                theme::ACCENT,
                i18n::t(
                    language,
                    "Downloading the attachment from SUNAT…",
                    "Descargando el adjunto desde SUNAT…",
                ),
                theme::MUTED,
            ),
            AttachmentPreviewState::Error(error) => centered_message(
                "alert-triangle",
                theme::DANGER,
                error,
                theme::DANGER,
            ),
            AttachmentPreviewState::Ready { content, .. } => match content {
                PreviewContent::Pdf { image: Some(path) } => image_preview(path),
                PreviewContent::Pdf { image: None } => centered_message(
                    "alert-circle",
                    theme::WARNING,
                    i18n::t(
                        language,
                        "This PDF could not be rendered. Download it to open it locally.",
                        "No se pudo renderizar este PDF. Descárgalo para abrirlo localmente.",
                    ),
                    theme::MUTED,
                ),
                PreviewContent::Image(path) => image_preview(path),
                PreviewContent::Text(text) => text_preview(text),
                PreviewContent::Unsupported => centered_message(
                    "mail",
                    theme::FAINT,
                    i18n::t(
                        language,
                        "Preview is not available for this file type. Download it to open it locally.",
                        "La vista previa no está disponible para este tipo de archivo. Descárgalo para abrirlo localmente.",
                    ),
                    theme::MUTED,
                ),
            },
        };

        let card = div()
            .flex()
            .flex_col()
            .w(px((width - 80.).clamp(360., 980.)))
            .h(relative(0.88))
            .bg(rgb(theme::SURFACE))
            .border_1()
            .border_color(rgb(theme::BORDER))
            .rounded_xl()
            .shadow_lg()
            .overflow_hidden()
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
                            .rounded_lg()
                            .bg(theme::tint(theme::ACCENT, 0.12))
                            .child(icon("mail", 17., theme::ACCENT)),
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
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(theme::TEXT))
                                    .text_ellipsis()
                                    .child(name),
                            )
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::FAINT))
                                    .child(format!("{customer} · {size}")),
                            ),
                    )
                    .children(ready.then(|| {
                        div()
                            .id("download-attachment")
                            .flex()
                            .items_center()
                            .gap_2()
                            .h(px(34.))
                            .px_3()
                            .rounded_lg()
                            .bg(rgb(theme::ACCENT))
                            .text_color(rgb(theme::BUTTON_TEXT))
                            .text_size(px(12.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .cursor_pointer()
                            .hover(|style| style.bg(rgb(theme::ACCENT_BRIGHT)))
                            .on_click(cx.listener(Self::download_attachment_preview))
                            .child(icon("download", 14., theme::BUTTON_TEXT))
                            .child(i18n::t(language, "Download", "Descargar"))
                            .into_any_element()
                    }))
                    .child(
                        icon_button("close-attachment-preview", "x")
                            .on_click(cx.listener(Self::close_attachment_preview)),
                    ),
            )
            .child(body);

        modal_overlay(card.into_any_element())
    }
}

fn image_preview(path: &std::path::Path) -> AnyElement {
    div()
        .flex_1()
        .min_h_0()
        .flex()
        .items_center()
        .justify_center()
        .p_4()
        .bg(rgb(theme::WHITE))
        .child(
            img(path.to_path_buf())
                .object_fit(ObjectFit::Contain)
                .w_full()
                .h_full(),
        )
        .into_any_element()
}

fn text_preview(text: &str) -> AnyElement {
    div()
        .id("attachment-text-scroll")
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .p_5()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children(text.lines().map(|line| {
                    div()
                        .text_size(px(11.5))
                        .line_height(px(17.))
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
                })),
        )
        .into_any_element()
}

fn centered_message(
    icon_name: &str,
    icon_color: u32,
    message: &str,
    text_color: u32,
) -> AnyElement {
    div()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .p_6()
        .child(icon(icon_name, 24., icon_color))
        .child(
            div()
                .text_size(px(12.5))
                .text_color(rgb(text_color))
                .child(message.to_string()),
        )
        .into_any_element()
}
