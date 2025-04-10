use eframe::egui::{ComboBox, TopBottomPanel, Ui};
use horae::TimeZone;

use crate::error::AnankeError;

use super::Ananke;

impl Ananke {

    pub fn menu_bar(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("About").clicked() {
                    self.state.error = Some(AnankeError::new("Test title", "Test message", Some("Test context")));
                };
                if ui.button("Load").clicked() {
                    self.load_file = true;
                }
                if ui.button("Save").clicked() {
                    self.save();
                };
                if ui.button("Save & Quit").clicked() {
                    self.save();
                    std::process::exit(0);
                };
                ui.separator();
                ui.add(|ui: &mut Ui| {
                    ComboBox::from_label("Timezone").selected_text(self.state.persistent_state.timezone.to_string()).show_ui(ui, |ui: &mut Ui| {
                        for tz in horae::TimeZone::get_all() {
                            if ui.selectable_value(&mut self.state.persistent_state.timezone, TimeZone::from(tz.clone()), tz.to_string()).clicked() {
                                self.save_state();
                            }
                        }
                    }).response
                });
            });
        });
    }

    pub fn save(&mut self) {
        self.save_state();
        if self.state.error.is_none() {
            match self.save_list() {
                Ok(()) => (),
                Err(e) => self.state.error = Some(e),
            }
        }
    }

    fn save_list(&mut self) -> Result<(), AnankeError> {
        todo!()
    }

    pub fn save_state(&mut self) {
        if let Err(e) = self.state.persistent_state.make_persistent() {
            self.state.error = Some(e);
        }
    }

}
