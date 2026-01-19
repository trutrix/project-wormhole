use bevy::{input_focus::tab_navigation::TabIndex, picking::hover::Hovered, prelude::*};

use crate::{DemoButton, ui::style::DEFAULT_BUTTON_BG};


#[derive(Component)]
pub struct ButtonNormal;

pub fn button_normal(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        ButtonNormal,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::BLACK),
        BackgroundColor(DEFAULT_BUTTON_BG),
        Interaction::default(),
        children![(
            Text::new("Button"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}