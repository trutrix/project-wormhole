use std::path::PathBuf;

use eframe::App;
use egui::*;

mod style;
use style::*;

mod strings;
use strings::*;

mod pages;

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

// ====================================================================================================

#[derive(Default)]
pub struct PWApp {
    game_path: Option<PathBuf>,
    app_state: PWAppState,
    page: PWAppPage,
    game_files: Vec<PWGameFile>
}

// ====================================================================================================

impl PWApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_global_style(Style {
            visuals: Visuals { dark_mode: true, ..Default::default() },
            ..Default::default()
        });
        Self::default()
    }

    pub fn init(&mut self) {
        if self.app_state == PWAppState::Startup {

            if let Some(path) = &self.game_path {
                unimplemented!("Possible to define, but not needed now.")
            } else {
                
            }



        } else {
            panic!("Undefined behavior: app init called when app is not in the startup state.")
        }
    }

    pub fn set_page(&mut self, page: PWAppPage) {
        // Extra page switching logic would go here
        self.page = page
    }

    pub fn set_game_path(&mut self, path: PathBuf) {
        self.game_path = Some(path);
        self.app_state = PWAppState::GameDirectoryChanged;
    }
}

// ====================================================================================================

impl eframe::App for PWApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        Panel::top("navbar")
        .show_separator_line(false)
        .frame(NAVBAR_FRAME)
        .show(ui, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.menu_button(RichText::new("☰"),
                |ui| {
                    
                    if ui.button(RichText::new(S_SET_GAME_DIRECTORY).color(COLOR_TEXT_LIGHT)).clicked() {
                        self.game_path = rfd::FileDialog::new().pick_folder();
                    }

                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                ui.menu_button(format!("👁 {:?}", self.page), |ui| {
                    if ui.button("Overview").clicked() {
                        self.set_page(PWAppPage::Overview);
                    }

                    if ui.button("Files").clicked() {
                        self.set_page(PWAppPage::Files);
                    }

                });

                
            });
        });

        Panel::bottom("status_bar")
        .show_separator_line(false)
        .frame(STATUS_BAR_FRAME)
        .show(ui, |ui| {
            // if let Some(gp) = &self.game_path {
            //     ui.label(gp.to_str().unwrap())
            // } else {
            //     ui.label(S_GAME_DIRECTORY_NOT_SET)
            // };

            ui.label(format!("AppState: {:?} | Page: {:?}", self.app_state, self.page));
        });

        CentralPanel::default().show(ui, |ui| {
            if self.game_path.is_some() {
                match self.page {
                    PWAppPage::Overview => { pages::Overview::add_page_contents(self, ui); },
                    PWAppPage::Files => { pages::Files::add_page_contents(self, ui); },
                }
            }
        });

        
    }
}

// ====================================================================================================

#[derive(Debug, Default, PartialEq, Eq)]
pub enum PWAppState {
    #[default]
    Startup,
    Idle,
    GameDirectoryChanged
}

// ====================================================================================================

#[derive(Debug, Default, PartialEq, Eq)]
pub enum PWAppPage {
    #[default]
    Overview,
    Files
}


// ====================================================================================================

#[derive(Debug)]
pub struct PWGameFile {
    pub enabled: bool,
    pub path: PathBuf
}

// ====================================================================================================

pub trait Page {
    fn add_page_contents(app: &mut PWApp, ui: &mut egui::Ui) {
        ui.label("Page contents not set");
    }
}