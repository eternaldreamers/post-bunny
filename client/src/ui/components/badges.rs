use gpui::{div, prelude::*, px, rgb, Div, FontWeight};

use crate::domain::alert::Severity;
use crate::domain::customer::LoginType;
use crate::domain::message::MessageKind;
use crate::ui::theme;

pub fn badge(label: &str, color: u32) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1()
        .h(px(22.))
        .px_2()
        .rounded_full()
        .bg(theme::tint(color, 0.13))
        .border_1()
        .border_color(theme::tint(color, 0.22))
        .text_color(rgb(color))
        .text_size(px(10.5))
        .font_weight(FontWeight::MEDIUM)
        .child(div().size(px(5.)).rounded_full().bg(rgb(color)).flex_none())
        .child(label.to_string())
}

pub fn plain_badge(label: &str, color: u32) -> Div {
    div()
        .flex()
        .items_center()
        .h(px(20.))
        .px_2()
        .rounded_full()
        .bg(theme::tint(color, 0.12))
        .text_color(rgb(color))
        .text_size(px(10.))
        .font_weight(FontWeight::MEDIUM)
        .child(label.to_string())
}

pub fn login_type_color(login_type: LoginType) -> u32 {
    match login_type {
        LoginType::Dni => theme::ACCENT,
        LoginType::Ruc => theme::PURPLE,
    }
}

pub fn message_kind_color(kind: MessageKind) -> u32 {
    match kind {
        MessageKind::Message => theme::ACCENT,
        MessageKind::Notification => theme::INFO,
    }
}

pub fn message_kind_icon(kind: MessageKind) -> &'static str {
    match kind {
        MessageKind::Message => "mail",
        MessageKind::Notification => "bell",
    }
}

pub fn severity_color(severity: Severity) -> u32 {
    match severity {
        Severity::Critical => theme::DANGER,
        Severity::Warning => theme::WARNING,
        Severity::Info => theme::INFO,
    }
}

pub fn severity_icon(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "alert-triangle",
        Severity::Warning => "alert-circle",
        Severity::Info => "info",
    }
}
