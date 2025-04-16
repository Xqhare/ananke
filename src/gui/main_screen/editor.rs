use anansi::Task;
use eframe::egui::{ComboBox, Grid, Ui, Vec2};

use crate::{gui::Ananke, util::NewTask};

const PRIOS: [&str; 27] = ["", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

impl Ananke {
    pub fn editor(&mut self, ui: &mut eframe::egui::Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                
                ui.horizontal(|ui| {
                    ComboBox::from_label("Priority").selected_text(self.new_task.prio.clone()).show_ui(ui, |ui: &mut Ui| {
                        for prio in PRIOS {
                            if ui.selectable_value(&mut self.new_task.prio, prio.to_string(), prio.to_string()).clicked() {
                                self.new_task.prio = prio.to_string();
                            }
                        }
                    });
                    ui.text_edit_singleline(&mut self.new_task.inception_date);
                    if ui.button("Create").clicked() {
                        self.entire_list.add(format!("({}) {} {}", self.new_task.prio, self.new_task.inception_date, self.new_task.text));
                        self.new_task = NewTask::new(self.state.persistent_state.timezone);
                    };
                    if ui.button("Reset").clicked() {
                        self.new_task = NewTask::new(self.state.persistent_state.timezone);
                    };
                });
                ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                    ui.text_edit_multiline(&mut self.new_task.text)
                });
            });
        });
    }
}
