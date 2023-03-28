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

pub const DOWNLOAD: char = '\u{f02e}';
pub const DOWN: char = '\u{e803}';
pub const LEFT: char = '\u{e804}';
pub const RIGHT: char = '\u{e805}';
pub const UP: char = '\u{e806}';
pub const DOWN_OPEN: char = '\u{f004}';
pub const RIGHT_OPEN: char = '\u{f006}';
pub const UP_OPEN: char = '\u{f005}';
pub const LEFT_OPEN: char = '\u{f007}';
pub const CCW: char = '\u{f025}';
pub const COG: char = '\u{e807}';
pub const OK: char = '\u{e800}';
pub const ATTENTION: char = '\u{e801}';
pub const ATTENTION_ALT: char = '\u{e802}';
pub const PLAY: char = '\u{f00f}';
pub const MENU: char = '\u{f008}';
pub const TH_THUMB: char = '\u{f00a}';
pub const CANCEL: char = '\u{e808}';
pub const WINDOW_MINIMIZE: char = '\u{f2d1}';
pub const WINDOW_MAXIMIZE: char = '\u{f2d0}';
pub const QUESTION_CIRCLE_O: char = '\u{3f}';
