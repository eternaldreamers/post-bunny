use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::customer::Customer;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{card, icon, icon_button, login_type_color};
use crate::ui::theme;

use super::widgets::credential_field;

impl Panel {
    pub(super) fn customer_credentials(
        &mut self,
        customer: &Customer,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let revealed = self.password_visible;
        let password = if revealed {
            customer.password.clone()
        } else {
            "•".repeat(customer.password.chars().count().min(22))
        };
        let login_color = login_type_color(customer.login_type);

        card()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .flex_wrap()
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
                                    .child(i18n::t(
                                        language,
                                        "SUNAT SOL credentials",
                                        "Credenciales SOL de SUNAT",
                                    )),
                            )
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::MUTED))
                                    .child(i18n::t(
                                        language,
                                        "Used to open the SUNAT mailbox for this customer.",
                                        "Se usan para abrir el buzón SUNAT de este cliente.",
                                    )),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_5()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .size(px(40.))
                                    .rounded_lg()
                                    .bg(theme::tint(login_color, 0.12))
                                    .child(icon("key-round", 18., login_color)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_size(px(13.))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child(i18n::login_mode_title(
                                                language,
                                                customer.login_type.label(),
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.))
                                            .text_color(rgb(theme::MUTED))
                                            .child(customer.login_type.description(language)),
                                    ),
                            )
                            .child(div().flex_1()),
                    )
                    .child(div().h(px(1.)).w_full().bg(rgb(theme::BORDER_SOFT)))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_6()
                            .child(credential_field(
                                i18n::t(language, "LOGIN TYPE", "TIPO DE ACCESO"),
                                customer.login_type.label(),
                            ))
                            .children(
                                customer
                                    .dni
                                    .as_ref()
                                    .map(|dni| credential_field("DNI", dni)),
                            )
                            .child(credential_field("RUC", &customer.ruc))
                            .children(customer.sol_user.as_ref().map(|user| {
                                credential_field(i18n::t(language, "SOL USER", "USUARIO SOL"), user)
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .min_w(px(220.))
                                    .child(
                                        div()
                                            .text_size(px(9.5))
                                            .text_color(rgb(theme::FAINT))
                                            .child(i18n::t(
                                                language,
                                                "SOL PASSWORD",
                                                "CONTRASEÑA SOL",
                                            )),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_size(px(12.))
                                                    .text_color(rgb(theme::TEXT))
                                                    .text_ellipsis()
                                                    .child(password),
                                            )
                                            .child(
                                                icon_button(
                                                    "toggle-password",
                                                    if revealed { "eye-off" } else { "eye" },
                                                )
                                                .on_click(
                                                    cx.listener(Self::toggle_password_visibility),
                                                ),
                                            ),
                                    ),
                            ),
                    )
                    .child(div().h(px(1.)).w_full().bg(rgb(theme::BORDER_SOFT)))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .text_size(px(10.5))
                            .text_color(rgb(theme::FAINT))
                            .child(icon("shield", 13., theme::FAINT))
                            .child(i18n::t(
                                language,
                                "Credentials are stored in this workspace.",
                                "Las credenciales se guardan en este espacio de trabajo.",
                            )),
                    ),
            )
            .into_any_element()
    }
}
