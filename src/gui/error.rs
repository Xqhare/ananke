use eframe::egui::CentralPanel;

use super::Ananke;

impl Ananke {
    pub fn error_screen(&mut self, ctx: &eframe::egui::Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.group(|ui|{
                ui.heading(self.state.error.as_ref().unwrap().title.as_str());
                ui.separator();
                let context = {
                    match &self.state.error.as_ref().unwrap().context {
                        Some(x) => x.as_str(),
                        None => "",
                    }
                };
                ui.label(context);
                ui.label(self.state.error.as_ref().unwrap().message.as_str());
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        self.state.error = None;
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    });
}
}
