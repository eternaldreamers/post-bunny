use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

pub struct Assets;

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        fn load_icon(path: &str) -> Option<&'static [u8]> {
            match path {
                $(concat!("icons/", $name, ".svg") => Some(include_bytes!(concat!(
                    "../../assets/icons/",
                    $name,
                    ".svg"
                ))),)*
                _ => None,
            }
        }
    };
}

icons!(
    "activity",
    "alert-circle",
    "alert-triangle",
    "arrow-left",
    "arrow-up-right",
    "bell",
    "building-2",
    "check",
    "chevron-down",
    "chevron-left",
    "chevron-right",
    "circle",
    "download",
    "circle-check",
    "clock",
    "copy",
    "credit-card",
    "external-link",
    "eye",
    "eye-off",
    "filter",
    "home",
    "id-card",
    "inbox",
    "info",
    "key-round",
    "languages",
    "log-out",
    "mail",
    "map-pin",
    "menu",
    "more-horizontal",
    "panel-left",
    "panel-left-close",
    "pencil",
    "phone",
    "plus",
    "rabbit",
    "refresh-cw",
    "search",
    "send",
    "settings",
    "shield",
    "sparkles",
    "trash-2",
    "triangle-alert",
    "user-plus",
    "users",
    "x",
);

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(load_icon(path).map(Cow::Borrowed))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
