use bevy::{input_focus::tab_navigation::TabGroup, prelude::*};
use bevy_ui_widgets::{Activate, ValueChange, checkbox_self_update, observe};

use crate::{DemoRadio, DemoWidgetStates, checkbox, radio_group, slider, style::*, ui::button::editor_button};

use crate::ui::navbar::navbar;

pub fn main_menu(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        TabGroup::default(),
        children![
            navbar(asset_server)
        ],
    )
}