use std::path::PathBuf;

use egui::*;

mod consts;
use consts::*;

mod strings;
use strings::*;

fn main() -> eframe::Result {
    eframe::run_native(
        APP_TITLE,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 720.0]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::<PWApp>::default()))
    )
}

#[derive(Default)]
struct PWApp {
    game_path: Option<PathBuf>,
    app_state: PWAppState,
}

impl eframe::App for PWApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {


        Panel::top("navbar")
        .show_separator_line(false)
        .frame(NAVBAR_FRAME)
        .show(ui, |ui| {
            ui.menu_button("Menu",
            |ui| {
                
                if ui.button(RichText::new("Set Game Directory...").color(COLOR_TEXT_LIGHT)).clicked() {
                    self.game_path = rfd::FileDialog::new().pick_folder();
                }
            });


            if let Some(gp) = &self.game_path {
                ui.label(gp.to_str().unwrap())
            } else {
                ui.label("Game not set")
            };
        });

        Panel::bottom("status_bar").show(ui, |ui| {
            ui.label("Bottom bar");
        });

        CentralPanel::default().show(ui, |ui| {
            ui.label("Holy poopy");
        });

        
    }
}



#[derive(Debug, Default)]
pub enum PWAppState {
    #[default]
    Startup,
    Idle,
    GameDirectoryChanged
}