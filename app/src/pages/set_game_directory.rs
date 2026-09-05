use egui::{Layout, panel};



pub struct SetGameDirectory;


impl crate::Page for SetGameDirectory {
    fn add_page_contents(_app: &mut crate::PWApp, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.label("Game directory not set.");
        });
    }
}