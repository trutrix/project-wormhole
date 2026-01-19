use bevy::{picking::hover::Hovered, prelude::*};
use crate::ui::style::*;

#[derive(Component)]
pub struct NavbarButton;

pub fn navbar_button(asset_server: &AssetServer, label: &str) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        NavbarButton,
        Hovered::default(),
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


pub fn update_navbar_button_style(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<NavbarButton>,
    >,
) {
    for (interaction, mut bg_color, mut border_color) in query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *bg_color = BackgroundColor(DEFAULT_BUTTON_BG);
                *border_color = BorderColor::all(DEFAULT_BUTTON_BORDER_COLOR);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(HOVERED_BUTTON);
            }
            Interaction::Pressed => {
                *bg_color = BackgroundColor(PRESSED_BUTTON);
            }
        }
    }
}