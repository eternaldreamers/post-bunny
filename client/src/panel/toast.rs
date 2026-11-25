use gpui::{ClickEvent, Context, Window};

use crate::config::TOAST_DURATION;

use super::{Panel, Toast, ToastKind};

impl Panel {
    pub fn dismiss_toast(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.toast = None;
        cx.notify();
    }

    pub fn show_toast(
        &mut self,
        message: impl Into<String>,
        kind: ToastKind,
        cx: &mut Context<Self>,
    ) {
        self.next_toast_id += 1;
        let id = self.next_toast_id;
        self.toast = Some(Toast {
            id,
            message: message.into(),
            kind,
        });
        cx.notify();

        let timer = cx.background_executor().timer(TOAST_DURATION);
        cx.spawn(async move |this, cx| {
            timer.await;
            this.update(cx, |panel, cx| {
                if panel.toast.as_ref().is_some_and(|toast| toast.id == id) {
                    panel.toast = None;
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }
}
