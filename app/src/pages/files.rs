use crate::Page;



pub struct Files;


impl Page for Files {
    fn add_page_contents(app: &mut crate::PWApp, ui: &mut egui::Ui) {
        ui.label("File browser");

        for game_file in &mut app.game_files {
            ui.checkbox(&mut game_file.enabled, format!("{:?}", game_file.path));
        }
    }
}