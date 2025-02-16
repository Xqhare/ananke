use eframe::egui::TopBottomPanel;

use super::Ananke;

impl Ananke {

    pub fn menu_bar(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if ui.button("About").clicked() {
                
            };
            if ui.button("Save").clicked() {
                self.save_state();
            };
        });
    }

    pub fn save(&mut self) {
        self.save_state();
        if self.state.error.is_none() {
            self.save_list();
        }
    }

    fn save_list(&mut self) {
        // TODO
    }

    pub fn save_state(&mut self) {
        if let Err(e) = self.state.persistent_state.make_persistent() {
            self.state.error = Some(e);
        }
    }

}
