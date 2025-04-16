mod load_file;
mod editor;

use eframe::egui::{CentralPanel, Grid, ScrollArea};

use super::Ananke;

impl Ananke {
    pub fn main_screen(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Grid::new("main_screen").num_columns(2).min_col_width(ui.available_width() / 2.5).max_col_width(ui.available_width() / 2.5).show(ui, |ui| {
                self.editor(ui);
                self.search(ui);
            });
            ScrollArea::vertical().show(ui, |ui| {
                self.task_list(ui);
            });
        });
    }

    fn search(&mut self, ui: &mut eframe::egui::Ui) {
        ui.group(|ui| {
            
            ui.label("TODO");
        });
    }

    fn task_list(&mut self, ui: &mut eframe::egui::Ui) {}
}
