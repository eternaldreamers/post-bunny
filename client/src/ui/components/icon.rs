use gpui::{prelude::*, px, rgb, svg, Svg};

pub fn icon(name: &str, size: f32, color: u32) -> Svg {
    svg()
        .path(format!("icons/{name}.svg"))
        .flex_none()
        .size(px(size))
        .text_color(rgb(color))
}
