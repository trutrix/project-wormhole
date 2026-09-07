use crate::Page;



pub struct Overview;


impl Page for Overview {
    fn add_page_contents(app: &mut crate::PWApp, ui: &mut egui::Ui) {
        ui.label("Overview");
    }
}