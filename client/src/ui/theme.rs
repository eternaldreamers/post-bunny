use gpui::{rgb, Hsla};

pub const BG: u32 = 0x14171d;
pub const SIDEBAR: u32 = 0x0f1217;
pub const SURFACE: u32 = 0x1a1e26;
pub const SURFACE_2: u32 = 0x20252f;
pub const SURFACE_3: u32 = 0x262c37;
pub const BORDER: u32 = 0x2a313d;
pub const BORDER_STRONG: u32 = 0x394252;
pub const BORDER_SOFT: u32 = 0x222833;
pub const TEXT: u32 = 0xe7eaf0;
pub const MUTED: u32 = 0x9aa3b2;
pub const FAINT: u32 = 0x6b7484;
pub const ACCENT: u32 = 0x74ade8;
pub const ACCENT_BRIGHT: u32 = 0x9cc7ff;
pub const SUCCESS: u32 = 0x6fd3a0;
pub const WARNING: u32 = 0xe8b563;
pub const DANGER: u32 = 0xef7d7d;
pub const DANGER_BRIGHT: u32 = 0xff9a9a;
pub const INFO: u32 = 0x8ab4f8;
pub const PURPLE: u32 = 0xb694f5;
pub const TEAL: u32 = 0x63c9c2;
pub const BUTTON_TEXT: u32 = 0x101319;
pub const DANGER_TEXT: u32 = 0x1b0d0d;
pub const OVERLAY_BACKDROP: u32 = 0x05070acc;
pub const OVERLAY_DRAWER: u32 = 0x05070ccc;
pub const WHITE: u32 = 0xffffff;
pub const SELECTION: u32 = 0x74ade83d;
pub const TRANSPARENT: u32 = 0x00000000;

pub fn tint(color: u32, alpha: f32) -> Hsla {
    let color: Hsla = rgb(color).into();
    color.opacity(alpha)
}

pub fn avatar_color(seed: u64) -> u32 {
    const COLORS: [u32; 6] = [ACCENT, PURPLE, TEAL, SUCCESS, WARNING, DANGER];
    COLORS[(seed as usize) % COLORS.len()]
}
