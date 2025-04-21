use eframe::egui::Ui;

use super::Ananke;

impl Ananke {
    pub fn task_list(&mut self, ui: &mut Ui) {
        for task in self.display_list.tasks() {
            ui.add_sized((ui.available_width(), 0.0), |ui: &mut Ui| {
                ui.group(|ui| {
                    ui.label(format!("{:?}", task));
                }).response
            });
        }
    }
}
