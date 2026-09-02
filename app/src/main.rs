use std::path::PathBuf;

use egui::*;

mod style;
use style::*;

mod strings;
use strings::*;

// ====================================================================================================

fn main() -> eframe::Result {
    eframe::run_native(
        S_APP_TITLE,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 720.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(PWApp::new(cc))))
    )
}

#[derive(Default)]
struct PWApp {
    game_path: Option<PathBuf>,
    app_state: PWAppState,
}

impl PWApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_global_style(Style {
            visuals: Visuals { dark_mode: true, ..Default::default() },
            ..Default::default()
        });
        Self::default()
    }
}

impl eframe::App for PWApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {


        Panel::top("navbar")
        .show_separator_line(false)
        .frame(NAVBAR_FRAME)
        .show(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.menu_button(S_MENU,
                |ui| {
                    
                    if ui.button(RichText::new(S_SET_GAME_DIRECTORY).color(COLOR_TEXT_LIGHT)).clicked() {
                        self.game_path = rfd::FileDialog::new().pick_folder();
                    }
                });

                ui.button("Huh")
            });
    
            


            
        });

        Panel::bottom("status_bar")
        .frame(STATUS_BAR_FRAME)
        .show(ui, |ui| {
            if let Some(gp) = &self.game_path {
                ui.label(gp.to_str().unwrap())
            } else {
                ui.label("Please set the game directory")
            };
        });

        CentralPanel::default().show(ui, |ui| {
            ui.label("Waiting")
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