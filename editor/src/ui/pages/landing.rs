use bevy::{input_focus::tab_navigation::TabGroup, prelude::*};
use bevy_ui_widgets::*;

use crate::ui::components::button::button_normal;

pub fn landing_page(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        TabGroup::default(),
        children![
            (
                button_normal(asset_server),
                observe(|_activate: On<Activate>| {
                    info!("Button clicked!");
                }),
            ),
            // (
            //     slider(0.0, 100.0, 50.0),
            //     observe(
            //         |value_change: On<ValueChange<f32>>,
            //          mut widget_states: ResMut<DemoWidgetStates>| {
            //             widget_states.slider_value = value_change.value;
            //         },
            //     )
            // ),
            // (
            //     checkbox(asset_server, "Checkbox"),
            //     observe(checkbox_self_update),
            // ),
            // Text::new("Press 'D' to toggle widget disabled states"),
        ],
    )
}
