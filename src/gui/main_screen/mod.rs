mod load_file;
mod editor;
mod search;

use eframe::egui::{CentralPanel, Grid, ScrollArea, Ui};

use super::Ananke;

impl Ananke {
    pub fn main_screen(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Grid::new("main_screen").num_columns(2).min_col_width(ui.available_width() / 2.7).max_col_width(ui.available_width() / 2.7).spacing((10.0, 10.0)).show(ui, |ui| {
                    self.editor(ui);
                    self.search(ui);
            });
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                self.task_list(ui);
            });
        });
    }

    fn task_list(&mut self, ui: &mut eframe::egui::Ui) {}
}
