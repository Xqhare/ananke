mod load_file;
mod editor;
mod search;
mod task_list;

use eframe::egui::{CentralPanel, Grid, ScrollArea, Ui};

use super::Ananke;

impl Ananke {
    pub fn main_screen(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Grid::new("main_screen").num_columns(2).show(ui, |ui| {
                self.search(ui);
                self.editor(ui);
            });
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                self.task_list(ui);
            });
        });
    }

}
