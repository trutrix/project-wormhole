use bevy::prelude::*;
use crate::style::*;

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
                font_size: NAVBAR_TEXT_SIZE_DEFAULT,
                ..default()
            },
            TextColor(DEFAULT_BUTTON_TEXT_COLOR),
            TextShadow::default(),
        )],
    )
}