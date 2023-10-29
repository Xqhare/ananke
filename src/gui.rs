use eframe::{run_native, App, egui::{CentralPanel, Ui}, NativeOptions};

#[derive(Default)]
struct TaskWidget;

impl App for TaskWidget {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Heading!");
            ui.label("Hello!");

        });
    }
}

pub fn main() {
    let app_name = "Ananke";
    let app_creator = TaskWidget;
    let native_options = NativeOptions::default();
    run_native(app_name, native_options, Box::new(|cc| {
        Box::<TaskWidget>::default()
    })).expect("E 001");
}
