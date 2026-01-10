use bevy::ecs::component::Component;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;

use crate::style::*;




#[derive(Component)]
pub struct EditorButton;



pub fn editor_button(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: px(80),
            height: px(30),
            border: UiRect::all(px(DEFAULT_BUTTON_BORDER_SIZE)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        EditorButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(DEFAULT_BUTTON_BORDER_COLOR),
        BorderRadius::all(px(0)),
        BackgroundColor(DEFAULT_BUTTON_BG),
        children![(
            Text::new("Button"),
            TextFont {
                font: asset_server.load(FONT_PATH),
                font_size: DEFAULT_BUTTON_TEXT_SIZE,
                ..default()
            },
            TextColor(DEFAULT_BUTTON_TEXT_COLOR),
            TextShadow::default(),
        )],
    )
}