use gpui::{prelude::*, px, size, App, Bounds, TitlebarOptions, WindowBounds, WindowOptions};

use crate::config;
use crate::i18n::{self, Language};
use crate::panel::Panel;

pub fn open(cx: &mut App, language: Language) {
    let (width, height) = config::initial_window_size();
    let bounds = Bounds::centered(None, size(px(width), px(height)), cx);
    let window = cx
        .open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(
                    px(config::MIN_WINDOW_WIDTH),
                    px(config::MIN_WINDOW_HEIGHT),
                )),
                titlebar: Some(TitlebarOptions {
                    title: Some(i18n::window_title(language).into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(Panel::new),
        )
        .expect("failed to open the panel window");

    window
        .update(cx, |panel, window, _cx| {
            window.focus(&panel.root_focus);
        })
        .ok();

    cx.activate(true);
}
