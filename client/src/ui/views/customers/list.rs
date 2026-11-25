use gpui::{div, prelude::*, px, rgb, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::alert::AlertState;
use crate::domain::customer::Customer;
use crate::domain::navigation::Page;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    avatar, card, element_id, empty_state, icon, login_type_color, plain_badge, primary_button,
};
use crate::ui::layout;
use crate::ui::theme;

impl Panel {
    pub(super) fn render_customer_list(
        &mut self,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let customers = self.filtered_customers(cx);
        let total = self.customers.len();
        let show_login = layout::show_customer_login(width);
        let show_messages = layout::show_customer_messages(width);
        let show_alerts = layout::show_customer_alerts(width);
        let show_last_sync = layout::show_customer_last_sync(width);
        let show_table_header = layout::show_customer_header(width);
        let search = self.customer_search.clone();
        let compact = layout::is_compact(width);

        let rows = customers
            .iter()
            .map(|customer| {
                self.customer_row(
                    customer,
                    show_login,
                    show_messages,
                    show_alerts,
                    show_last_sync,
                    cx,
                )
            })
            .collect::<Vec<_>>();

        let list_body = if rows.is_empty() {
            empty_state(
                "users",
                i18n::t(
                    language,
                    "No customers match this view",
                    "Ningún cliente coincide con esta vista",
                ),
                i18n::t(
                    language,
                    "Try a different search term.",
                    "Prueba con otro término de búsqueda.",
                ),
            )
            .into_any_element()
        } else {
            let mut table = div().flex().flex_col();
            if show_table_header {
                table = table.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_4()
                        .h(px(34.))
                        .px_4()
                        .border_b_1()
                        .border_color(rgb(theme::BORDER_SOFT))
                        .bg(rgb(theme::SURFACE_2))
                        .text_size(px(9.5))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme::FAINT))
                        .child(
                            div()
                                .flex_1()
                                .min_w(px(220.))
                                .child(i18n::t(language, "CUSTOMER", "CLIENTE")),
                        )
                        .children(show_login.then(|| {
                            div()
                                .w(px(170.))
                                .child(i18n::t(language, "LOGIN", "ACCESO"))
                        }))
                        .children(show_messages.then(|| {
                            div()
                                .w(px(100.))
                                .child(i18n::t(language, "MESSAGES", "MENSAJES"))
                        }))
                        .children(show_alerts.then(|| {
                            div()
                                .w(px(70.))
                                .child(i18n::t(language, "ALERTS", "ALERTAS"))
                        }))
                        .children(show_last_sync.then(|| {
                            div()
                                .w(px(130.))
                                .child(i18n::t(language, "LAST SYNC", "ÚLTIMA SINCR."))
                        }))
                        .child(div().w(px(20.))),
                );
            }
            table.children(rows).into_any_element()
        };

        div()
            .id("customers-scroll")
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
                                                    .child(Page::Customers.title(language)),
                                            )
                                            .child(plain_badge(
                                                &i18n::accounts(language, total),
                                                theme::MUTED,
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.5))
                                            .text_color(rgb(theme::MUTED))
                                            .child(Page::Customers.subtitle(language)),
                                    ),
                            )
                            .child(
                                primary_button(
                                    "new-customer-page",
                                    i18n::t(language, "New customer", "Nuevo cliente"),
                                )
                                .on_click(cx.listener(Self::open_new_customer)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_3()
                            .flex_wrap()
                            .child(div().flex_1().min_w(px(240.)).max_w(px(440.)).child(search))
                            .child(
                                div()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::FAINT))
                                    .child(i18n::showing_customers(
                                        language,
                                        customers.len(),
                                        total,
                                    )),
                            ),
                    )
                    .child(card().child(list_body)),
            )
            .into_any_element()
    }

    fn customer_row(
        &self,
        customer: &Customer,
        show_login: bool,
        show_messages: bool,
        show_alerts: bool,
        show_last_sync: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let id = customer.id;
        let open_alerts = self
            .alerts
            .iter()
            .filter(|alert| alert.customer_id == id && alert.state != AlertState::Resolved)
            .count();
        let unread = customer.unread_count();
        let login_color = login_type_color(customer.login_type);

        div()
            .id(element_id(format!("customer-row-{}", customer.id)))
            .flex()
            .items_center()
            .gap_4()
            .h(px(58.))
            .px_4()
            .border_b_1()
            .border_color(rgb(theme::BORDER_SOFT))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(theme::SURFACE_2)))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.open_customer_by_id(id, cx);
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .flex_1()
                    .min_w_0()
                    .child(avatar(&customer.initials(), customer.id, 34.))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(1.))
                            .min_w_0()
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(theme::TEXT))
                                    .text_ellipsis()
                                    .child(customer.title()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .text_size(px(10.5))
                                    .text_color(rgb(theme::MUTED))
                                    .child(format!("RUC {}", customer.ruc)),
                            ),
                    ),
            )
            .children(show_login.then(|| {
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .w(px(170.))
                    .child(plain_badge(customer.login_type.label(), login_color))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(theme::MUTED))
                            .text_ellipsis()
                            .child(
                                customer
                                    .sol_user
                                    .clone()
                                    .unwrap_or_else(|| customer.login_identifier().to_string()),
                            ),
                    )
            }))
            .children(show_messages.then(|| {
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .w(px(100.))
                    .child(icon(
                        "mail",
                        12.,
                        if unread > 0 {
                            theme::ACCENT
                        } else {
                            theme::FAINT
                        },
                    ))
                    .child(
                        div()
                            .text_size(px(11.))
                            .font_weight(if unread > 0 {
                                FontWeight::MEDIUM
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(rgb(if unread > 0 {
                                theme::TEXT
                            } else {
                                theme::MUTED
                            }))
                            .child(format!("{}/{}", unread, customer.message_count())),
                    )
            }))
            .children(show_alerts.then(|| {
                div()
                    .w(px(70.))
                    .text_size(px(11.))
                    .font_weight(if open_alerts > 0 {
                        FontWeight::MEDIUM
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_color(rgb(if open_alerts > 0 {
                        theme::WARNING
                    } else {
                        theme::FAINT
                    }))
                    .child(open_alerts.to_string())
            }))
            .children(show_last_sync.then(|| {
                div()
                    .w(px(130.))
                    .text_size(px(10.5))
                    .text_color(rgb(theme::MUTED))
                    .text_ellipsis()
                    .child(
                        customer
                            .last_sync
                            .clone()
                            .unwrap_or_else(|| i18n::t(language, "Never", "Nunca").into()),
                    )
            }))
            .child(div().w(px(20.)).flex().justify_end().child(icon(
                "chevron-right",
                14.,
                theme::FAINT,
            )))
            .into_any_element()
    }
}
