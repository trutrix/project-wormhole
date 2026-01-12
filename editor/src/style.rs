use bevy::{color::Color, ui::{UiRect, px}};


pub const FONT_PATH: &str = "fonts/Inconsolata-Var.ttf";

pub const DEFAULT_TEXT_SIZE: f32 = 16.0;



pub const DEFAULT_BUTTON_BG: Color = Color::srgb(0.15, 0.15, 0.15);
pub const DEFAULT_BUTTON_BORDER_SIZE: i32 = 1;
pub const DEFAULT_BUTTON_BORDER_COLOR: Color = Color::srgb(0.45, 0.45, 0.45);
pub const DEFAULT_BUTTON_TEXT_SIZE: f32 = DEFAULT_TEXT_SIZE;
pub const DEFAULT_BUTTON_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);



pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
pub const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
pub const ELEMENT_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);
pub const ELEMENT_FILL: Color = Color::srgb(0.35, 0.75, 0.35);
pub const ELEMENT_FILL_DISABLED: Color = Color::srgb(0.5019608, 0.5019608, 0.5019608);



pub const NAVBAR_COLOR_BG: Color = Color::srgb(0.1, 0.1, 0.12);
pub const NAVBAR_TEXT_SIZE_DEFAULT: f32 = 14.0;