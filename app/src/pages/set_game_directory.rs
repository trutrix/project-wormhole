

pub struct SetGameDirectory {

}


impl crate::Page for SetGameDirectory {
    fn add_content(app: &mut crate::PWApp, ui: &mut egui::Ui) -> egui::Response {
        ui.label("Set Game Directory")
    }
}