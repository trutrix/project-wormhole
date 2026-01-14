pub mod navbar_button;
use bevy_ui_widgets::{Activate, observe};
use navbar_button::navbar_button;

use crate::style::*;
use bevy::{prelude::*};



pub fn navbar(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: percent(100),
            border: UiRect::bottom(px(1)),
            padding: UiRect::all(px(5)),
            ..Default::default()
        },
        BackgroundColor(NAVBAR_COLOR_BG),
        children![
            (
                navbar_button(asset_server, "File"),
                observe(|_a: On<Activate>| {
                    info!("File menu clicked!");
                    println!("File menu clicked!");
                }),
            )
        ]
    )
}