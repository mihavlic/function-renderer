/// character definitions for our bundled icon font
/// generated with https://fontello.com/
use std::sync::Arc;

pub fn egui_icon_font_family() -> egui::FontFamily {
    egui::FontFamily::Name("icons".into())
}

pub const FONT_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/icon-font/fontello.ttf"
));

pub const DOWNLOAD: char = '\u{F02E}';
pub const DOWN: char = '\u{E803}';
pub const LEFT: char = '\u{E804}';
pub const RIGHT: char = '\u{E805}';
pub const UP: char = '\u{E806}';
pub const DOWN_OPEN: char = '\u{F004}';
pub const RIGHT_OPEN: char = '\u{F006}';
pub const UP_OPEN: char = '\u{F005}';
pub const LEFT_OPEN: char = '\u{F007}';
pub const CCW: char = '\u{F025}';
pub const COG: char = '\u{E807}';
pub const OK: char = '\u{E800}';
pub const ATTENTION: char = '\u{E801}';
pub const ATTENTION_ALT: char = '\u{E802}';
pub const PLAY: char = '\u{F00F}';
pub const MENU: char = '\u{F008}';
pub const TH_THUMB: char = '\u{F00A}';
