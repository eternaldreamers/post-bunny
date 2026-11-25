use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{danger_button, icon, secondary_button};
use crate::ui::theme;

use super::modal::{modal_card, modal_overlay};

impl Panel {
    pub(super) fn render_delete_customer(
        &mut self,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let Some(id) = self.delete_customer else {
            return div().into_any_element();
        };
        let Some(customer) = self.customer(id) else {
            return div().into_any_element();
        };
        let name = customer.title();
        let ruc = customer.ruc.clone();

        let card = modal_card(width, 320., 420.)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .p_6()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(44.))
                            .rounded_full()
                            .bg(theme::tint(theme::DANGER, 0.12))
                            .child(icon("trash-2", 20., theme::DANGER)),
                    )
                    .child(
                        div()
                            .text_size(px(16.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(format!(
                                "{} {name}?",
                                i18n::t(language, "Delete", "¿Eliminar a")
                            )),
                    )
                    .child(
                        div()
                            .text_center()
                            .text_size(px(12.))
                            .text_color(rgb(theme::MUTED))
                            .child(
                                i18n::t(
                                    language,
                                    "This removes the stored SOL credentials, mailbox cache and related alerts for RUC {ruc}. This action cannot be undone.",
                                    "Esto elimina las credenciales SOL, la caché del buzón y las alertas relacionadas del RUC {ruc}. Esta acción no se puede deshacer.",
                                )
                                .replace("{ruc}", &ruc),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .p_4()
                    .border_t_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .child(
                        secondary_button(
                            "cancel-delete",
                            i18n::t(language, "Cancel", "Cancelar"),
                        )
                        .flex_1()
                        .on_click(cx.listener(Self::cancel_delete_customer)),
                    )
                    .child(
                        danger_button(
                            "confirm-delete",
                            i18n::t(language, "Delete customer", "Eliminar cliente"),
                        )
                        .flex_1()
                        .on_click(cx.listener(Self::confirm_delete_customer)),
                    ),
            );

        modal_overlay(card.into_any_element())
    }
}
