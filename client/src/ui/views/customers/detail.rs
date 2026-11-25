use gpui::{div, prelude::*, px, rgb, rgba, AnyElement, Context, FontWeight, IntoElement, Styled};

use crate::domain::customer::Customer;
use crate::domain::navigation::DetailTab;
use crate::i18n;
use crate::panel::Panel;
use crate::ui::components::{
    avatar, card, element_id, icon, login_type_color, plain_badge, secondary_button,
};
use crate::ui::layout;
use crate::ui::theme;

use super::widgets::meta_item;

impl Panel {
    pub(super) fn render_customer_detail(
        &mut self,
        customer: Customer,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let language = self.language;
        let customer_id = customer.id;
        let compact = layout::is_detail_compact(width);
        let unread = customer.unread_count();
        let alerts = self.customer_alerts(customer_id);

        let header = card()
            .p_5()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .gap_4()
                    .flex_wrap()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(avatar(&customer.initials(), customer.id, 56.))
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
                                            .flex_wrap()
                                            .child(
                                                div()
                                                    .text_size(px(22.))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(theme::TEXT))
                                                    .child(customer.title()),
                                            )
                                            .child(plain_badge(
                                                customer.login_type.label(),
                                                login_type_color(customer.login_type),
                                            )),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .text_size(px(12.))
                                            .text_color(rgb(theme::MUTED))
                                            .child(format!("RUC {}", customer.ruc))
                                            .when(customer.display_name.is_none(), |this| {
                                                this.child(format!(
                                                    "· {}",
                                                    i18n::t(
                                                        language,
                                                        "Awaiting first SUNAT sync",
                                                        "Esperando la primera sincronización SUNAT",
                                                    )
                                                ))
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                secondary_button(
                                    "sync-customer",
                                    if self.syncing {
                                        i18n::t(language, "Syncing…", "Sincronizando…")
                                    } else {
                                        i18n::t(language, "Sync", "Sincronizar")
                                    },
                                )
                                .child(icon(
                                    "refresh-cw",
                                    14.,
                                    if self.syncing {
                                        theme::FAINT
                                    } else {
                                        theme::MUTED
                                    },
                                ))
                                .when(self.syncing, |this| this.opacity(0.55))
                                .when(!self.syncing, |this| {
                                    this.on_click(cx.listener(move |this, _, _, cx| {
                                        this.sync_one(customer_id, cx);
                                    }))
                                }),
                            )
                            .child(
                                secondary_button(
                                    "edit-customer",
                                    i18n::t(language, "Edit credentials", "Editar credenciales"),
                                )
                                .on_click(cx.listener(
                                    move |this, _, window, cx| {
                                        this.open_edit_customer(customer_id, window, cx);
                                    },
                                )),
                            )
                            .child(
                                secondary_button(
                                    "delete-customer",
                                    i18n::t(language, "Delete", "Eliminar"),
                                )
                                .on_click(cx.listener(
                                    move |this, _, _, cx| {
                                        this.ask_delete_customer(customer_id, cx);
                                    },
                                )),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_5()
                    .text_size(px(11.5))
                    .text_color(rgb(theme::MUTED))
                    .child(meta_item("building-2", &format!("RUC {}", customer.ruc)))
                    .children(
                        customer
                            .dni
                            .as_ref()
                            .map(|dni| meta_item("id-card", &format!("DNI {dni}"))),
                    )
                    .children(customer.sol_user.as_ref().map(|user| {
                        meta_item(
                            "user-plus",
                            &format!("{} {user}", i18n::t(language, "SOL user", "Usuario SOL")),
                        )
                    }))
                    .child(meta_item(
                        "refresh-cw",
                        &format!(
                            "{} {}",
                            i18n::t(language, "Last sync", "Última sincronización"),
                            customer
                                .last_sync
                                .as_deref()
                                .unwrap_or(i18n::t(language, "never", "nunca"))
                        ),
                    )),
            );

        let tab = |id: &str,
                   label: &str,
                   count: Option<usize>,
                   active: bool,
                   target: DetailTab,
                   cx: &mut Context<Self>| {
            let mut element = div()
                .id(element_id(id.to_string()))
                .flex()
                .items_center()
                .gap_2()
                .pb_3()
                .border_b_2()
                .cursor_pointer()
                .text_size(px(12.))
                .font_weight(FontWeight::MEDIUM)
                .when(active, |this| {
                    this.border_color(rgb(theme::ACCENT))
                        .text_color(rgb(theme::TEXT))
                })
                .when(!active, |this| {
                    this.border_color(rgba(theme::TRANSPARENT))
                        .text_color(rgb(theme::MUTED))
                        .hover(|style| style.text_color(rgb(theme::TEXT)))
                })
                .child(label.to_string());
            if let Some(count) = count {
                element = element.child(plain_badge(
                    &count.to_string(),
                    if active { theme::ACCENT } else { theme::FAINT },
                ));
            }
            if !active {
                element = element.on_click(cx.listener(move |this, _, _, cx| {
                    this.select_detail_tab(target, cx);
                }));
            }
            element
        };

        let tabs = div()
            .flex()
            .items_center()
            .gap_6()
            .border_b_1()
            .border_color(rgb(theme::BORDER_SOFT))
            .child(tab(
                "tab-overview",
                DetailTab::Overview.label(language),
                None,
                self.detail_tab == DetailTab::Overview,
                DetailTab::Overview,
                cx,
            ))
            .child(tab(
                "tab-credentials",
                DetailTab::Credentials.label(language),
                None,
                self.detail_tab == DetailTab::Credentials,
                DetailTab::Credentials,
                cx,
            ))
            .child(tab(
                "tab-inbox",
                DetailTab::Inbox.label(language),
                Some(unread),
                self.detail_tab == DetailTab::Inbox,
                DetailTab::Inbox,
                cx,
            ));

        let body = match self.detail_tab {
            DetailTab::Overview => self.customer_overview(&customer, &alerts, cx),
            DetailTab::Credentials => self.customer_credentials(&customer, cx),
            DetailTab::Inbox => self.customer_inbox(&customer, width, cx),
        };

        div()
            .id("customer-detail-scroll")
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
                    .child(tabs)
                    .child(body),
            )
            .into_any_element()
    }
}
