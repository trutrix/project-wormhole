use egui::*;


fn main() -> eframe::Result {
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "ProjectWormholeApp",
        options,
        Box::new(|_cc| Ok(Box::<PWApp>::default())),
    )
}

#[derive(Default)]
struct PWApp {
    dropped_files: Vec<egui::DroppedFileHandle>,
    picked_path: Option<String>,
}

impl eframe::App for PWApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        egui::Panel::top("nav_bar").show(ui, |ui| {
            ui.label("Top bar");
        });

        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.label("Bottom bar");
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Holy poopy");
        });

        
    }
}