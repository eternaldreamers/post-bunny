use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::customer::LoginType;
use crate::domain::identity::{is_valid_ruc, ruc_from_dni};
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    field_label, icon, icon_button, labeled_input, primary_button, secondary_button,
};
use crate::ui::layout;
use crate::ui::theme;

use super::modal::{modal_card, modal_header, modal_overlay};

impl Panel {
    pub(super) fn render_customer_form(
        &mut self,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let editing = self.form.editing.is_some();
        let login_type = self.form.login_type;
        let wide = layout::is_form_wide(width);
        let derived_ruc = ruc_from_dni(self.form.dni.read(cx).text().trim());
        let typed_ruc = self.form.ruc.read(cx).text().trim().to_string();

        let dni = self.form.dni.clone();
        let ruc = self.form.ruc.clone();
        let sol_user = self.form.sol_user.clone();
        let password = self.form.password.clone();
        let show_password = self.form.show_password;

        let mode_button = |login: LoginType,
                           id: &'static str,
                           icon_name: &'static str,
                           active: bool,
                           cx: &mut Context<Self>| {
            let mut button = div()
                .id(id)
                .flex()
                .items_center()
                .gap_2()
                .h(px(40.))
                .px_3()
                .rounded_lg()
                .border_1()
                .cursor_pointer()
                .text_size(px(12.5))
                .font_weight(FontWeight::MEDIUM)
                .when(active, |this| {
                    this.bg(theme::tint(theme::ACCENT, 0.12))
                        .border_color(theme::tint(theme::ACCENT, 0.4))
                        .text_color(rgb(theme::ACCENT_BRIGHT))
                })
                .when(!active, |this| {
                    this.bg(rgb(theme::SURFACE_2))
                        .border_color(rgb(theme::BORDER))
                        .text_color(rgb(theme::MUTED))
                        .hover(|style| style.bg(rgb(theme::SURFACE_3)).text_color(rgb(theme::TEXT)))
                })
                .child(icon(
                    icon_name,
                    15.,
                    if active { theme::ACCENT } else { theme::FAINT },
                ))
                .child(login.label());
            if !active {
                button = button.on_click(cx.listener(move |this, _, window, cx| {
                    this.set_login_type(login, window, cx);
                }));
            }
            button
        };

        let password_field = div()
            .flex()
            .flex_col()
            .gap_2()
            .min_w_0()
            .child(field_label(i18n::t(
                language,
                "SOL password",
                "Contraseña SOL",
            )))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().flex_1().min_w_0().child(password))
                    .child(
                        icon_button(
                            "toggle-form-password",
                            if show_password { "eye-off" } else { "eye" },
                        )
                        .on_click(cx.listener(Self::toggle_form_password)),
                    ),
            );

        let identity_fields = match login_type {
            LoginType::Dni => div()
                .flex()
                .flex_col()
                .gap_4()
                .child(if wide {
                    div()
                        .flex()
                        .gap_3()
                        .child(div().flex_1().child(labeled_input("DNI", dni.clone())))
                        .child(div().flex_1().child(password_field))
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(labeled_input("DNI", dni.clone()))
                        .child(password_field)
                        .into_any_element()
                })
                .child(match &derived_ruc {
                    Some(ruc) => div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_size(px(11.))
                        .text_color(rgb(theme::SUCCESS))
                        .child(icon("check", 13., theme::SUCCESS))
                        .child(format!(
                            "{} {ruc}",
                            i18n::t(language, "Derived RUC:", "RUC derivado:")
                        ))
                        .into_any_element(),
                    None => div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_size(px(11.))
                        .text_color(rgb(theme::FAINT))
                        .child(icon("info", 13., theme::FAINT))
                        .child(i18n::t(
                            language,
                            "Enter the 8-digit DNI; the RUC is derived automatically.",
                            "Ingresa el DNI de 8 dígitos; el RUC se calcula automáticamente.",
                        ))
                        .into_any_element(),
                }),
            LoginType::Ruc => div()
                .flex()
                .flex_col()
                .gap_4()
                .child(if wide {
                    div()
                        .flex()
                        .gap_3()
                        .child(div().flex_1().child(labeled_input("RUC", ruc.clone())))
                        .child(div().flex_1().child(labeled_input(
                            i18n::t(language, "SOL user", "Usuario SOL"),
                            sol_user.clone(),
                        )))
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .child(labeled_input("RUC", ruc.clone()))
                        .child(labeled_input(
                            i18n::t(language, "SOL user", "Usuario SOL"),
                            sol_user.clone(),
                        ))
                        .into_any_element()
                })
                .child(password_field)
                .child(match &typed_ruc {
                    value if value.is_empty() => div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_size(px(11.))
                        .text_color(rgb(theme::FAINT))
                        .child(icon("info", 13., theme::FAINT))
                        .child(i18n::t(
                            language,
                            "Enter the 11-digit RUC, the SOL user and the SOL password.",
                            "Ingresa el RUC de 11 dígitos, el usuario SOL y la contraseña SOL.",
                        ))
                        .into_any_element(),
                    value if is_valid_ruc(value) => div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_size(px(11.))
                        .text_color(rgb(theme::SUCCESS))
                        .child(icon("check", 13., theme::SUCCESS))
                        .child(i18n::t(
                            language,
                            "RUC check digit is valid.",
                            "El dígito verificador del RUC es válido.",
                        ))
                        .into_any_element(),
                    _ => div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_size(px(11.))
                        .text_color(rgb(theme::DANGER))
                        .child(icon("alert-circle", 13., theme::DANGER))
                        .child(i18n::t(
                            language,
                            "The RUC check digit does not match.",
                            "El dígito verificador del RUC no coincide.",
                        ))
                        .into_any_element(),
                }),
        };

        let body = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_5()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(field_label(i18n::t(
                        language,
                        "Login type",
                        "Tipo de acceso",
                    )))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(mode_button(
                                LoginType::Dni,
                                "login-dni",
                                "id-card",
                                login_type == LoginType::Dni,
                                cx,
                            ))
                            .child(mode_button(
                                LoginType::Ruc,
                                "login-ruc",
                                "building-2",
                                login_type == LoginType::Ruc,
                                cx,
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(10.5))
                            .text_color(rgb(theme::FAINT))
                            .child(login_type.description(language)),
                    ),
            )
            .child(identity_fields)
            .children(self.form.error.map(|error| {
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .rounded_lg()
                    .bg(theme::tint(theme::DANGER, 0.1))
                    .border_1()
                    .border_color(theme::tint(theme::DANGER, 0.25))
                    .text_size(px(11.5))
                    .text_color(rgb(theme::DANGER))
                    .child(icon("alert-circle", 14., theme::DANGER))
                    .child(error)
            }));

        let card = modal_card(width, 340., 560.)
            .child(modal_header(
                if editing {
                    i18n::t(
                        language,
                        "Edit SUNAT credentials",
                        "Editar credenciales SUNAT",
                    )
                } else {
                    i18n::t(language, "New SUNAT customer", "Nuevo cliente SUNAT")
                },
                i18n::t(
                    language,
                    "A customer is a SUNAT user with a SOL login.",
                    "Un cliente es un usuario SUNAT con acceso SOL.",
                ),
                cx.listener(Self::close_customer_form),
            ))
            .child(
                div()
                    .id("customer-form-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(body),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .p_4()
                    .border_t_1()
                    .border_color(rgb(theme::BORDER_SOFT))
                    .child(
                        secondary_button(
                            "cancel-customer",
                            i18n::t(language, "Cancel", "Cancelar"),
                        )
                        .on_click(cx.listener(Self::close_customer_form)),
                    )
                    .child(
                        primary_button(
                            "save-customer",
                            if editing {
                                i18n::t(language, "Save credentials", "Guardar credenciales")
                            } else {
                                i18n::t(language, "Add customer", "Agregar cliente")
                            },
                        )
                        .on_click(cx.listener(Self::save_customer)),
                    ),
            );

        modal_overlay(card.into_any_element())
    }
}
