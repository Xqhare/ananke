use eframe::egui::TopBottomPanel;

use super::Ananke;

impl Ananke {

    pub fn menu_bar(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if ui.button("About").clicked() {
                
            };
        });
    }

}
