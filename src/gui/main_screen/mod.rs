
use eframe::egui::{CentralPanel, ScrollArea};

use super::Ananke;

impl Ananke {
    pub fn main_screen(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.editor(ui);
            ScrollArea::vertical().show(ui, |ui| {
                self.task_list(ui);
            });
        });
    }

    fn editor(&mut self, ui: &mut eframe::egui::Ui) {}

    fn task_list(&mut self, ui: &mut eframe::egui::Ui) {}
}
