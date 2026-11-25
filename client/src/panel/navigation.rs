use gpui::{ClickEvent, Context, Window};

use crate::domain::navigation::{DetailTab, Page};

use super::Panel;

impl Panel {
    pub fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        self.page = page;
        self.drawer_open = false;
        cx.notify();
    }

    pub fn nav_home(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.navigate(Page::Home, cx);
    }

    pub fn nav_customers(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.page == Page::Customers {
            self.selected_customer_id = None;
            self.reading_message_id = None;
        }
        self.navigate(Page::Customers, cx);
    }

    pub fn nav_alerts(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.navigate(Page::Alerts, cx);
    }

    pub fn toggle_drawer(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.drawer_open = !self.drawer_open;
        cx.notify();
    }

    pub fn close_drawer(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.drawer_open = false;
        cx.notify();
    }

    pub fn go_customers_list(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_customer_id = None;
        self.reading_message_id = None;
        self.navigate(Page::Customers, cx);
    }

    pub fn open_customer_by_id(&mut self, id: u64, cx: &mut Context<Self>) {
        self.selected_customer_id = Some(id);
        self.detail_tab = DetailTab::Overview;
        self.reading_message_id = None;
        self.password_visible = false;
        self.page = Page::Customers;
        cx.notify();
    }

    pub fn back_to_customers(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_customer_id = None;
        self.reading_message_id = None;
        cx.notify();
    }

    pub fn select_detail_tab(&mut self, tab: DetailTab, cx: &mut Context<Self>) {
        self.detail_tab = tab;
        self.reading_message_id = None;
        self.password_visible = false;
        cx.notify();
    }
}
