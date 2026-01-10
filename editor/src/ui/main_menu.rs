use bevy::{input_focus::tab_navigation::TabGroup, prelude::*};
use bevy_ui_widgets::{Activate, ValueChange, checkbox_self_update, observe};

use crate::{DemoRadio, DemoWidgetStates, checkbox, radio_group, slider, style::*, ui::button::editor_button};


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

pub fn navbar(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: percent(100),
            border: UiRect::bottom(px(1)),
            padding: UiRect::all(px(5)),
            ..Default::default()


        },
        BackgroundColor(Color::Srgba(Srgba { red: 0.1, green: 0.1, blue: 0.12, alpha: 1.0 })),
        children![
            navbar_button(asset_server, "File"),
        ]
    )
}


pub fn navbar_button(asset_server: &AssetServer, label: &str) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        children![(
            Text::new(label),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: 14.0,
                ..default()
            },
            TextColor(DEFAULT_BUTTON_TEXT_COLOR),
            TextShadow::default(),
        )],
    )
}