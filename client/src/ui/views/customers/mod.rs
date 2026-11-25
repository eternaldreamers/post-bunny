mod credentials;
mod detail;
mod inbox;
mod list;
mod overview;
mod reading;
mod widgets;

use gpui::{AnyElement, Context};

use crate::panel::Panel;

impl Panel {
    pub(crate) fn render_customers(&mut self, width: f32, cx: &mut Context<Self>) -> AnyElement {
        match self.selected_customer().cloned() {
            Some(customer) => self.render_customer_detail(customer, width, cx),
            None => self.render_customer_list(width, cx),
        }
    }
}
