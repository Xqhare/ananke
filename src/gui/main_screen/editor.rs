use eframe::egui::{ComboBox, Response, Ui};

use crate::{gui::Ananke, util::NewTask};

pub const PRIOS: [&str; 27] = ["", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

impl Ananke {
    pub fn editor(&mut self, ui: &mut eframe::egui::Ui) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ComboBox::from_id_salt("prio").selected_text(self.new_task.prio.clone()).show_ui(ui, |ui: &mut Ui| {
                        for prio in PRIOS {
                            if ui.selectable_value(&mut self.new_task.prio, prio.to_string(), prio.to_string()).clicked() {
                               self.new_task.prio = prio.to_string();
                            }
                        }
                    });
                    ui.add_enabled(self.state.gui_state.editor_gui_state.edit_date, |ui: &mut Ui| {
                        ui.add_sized((75.0, ui.available_height()), |ui: &mut Ui| {
                            ui.text_edit_singleline(&mut self.new_task.inception_date)
                        })
                    });
                    if self.state.gui_state.editor_gui_state.edit_date {
                        if ui.button("Lock date").clicked() {
                            self.state.gui_state.editor_gui_state.edit_date = false;
                        }
                    } else {
                        if ui.button("Edit date").clicked() {
                            self.state.gui_state.editor_gui_state.edit_date = true;
                        }
                    }
                    if !self.state.gui_state.editor_gui_state.confirm_reset {
                        if ui.button("Reset").clicked() {
                            self.state.gui_state.editor_gui_state.confirm_reset = true;
                        };
                    } else {
                        ui.separator();
                        if ui.button("Confirm").clicked() {
                            self.state.gui_state.editor_gui_state.confirm_reset = false;
                            self.new_task = NewTask::new(self.state.persistent_state.timezone);
                        };
                        if ui.button("Cancel").clicked() {
                            self.state.gui_state.editor_gui_state.confirm_reset = false;
                        };
                        ui.separator();
                    }
                    if ui.button("Create").clicked() {
                        self.entire_list.add(format!("({}) {} {}", self.new_task.prio, self.new_task.inception_date, self.new_task.text));
                        self.new_task = NewTask::new(self.state.persistent_state.timezone);
                    };
                });
                ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                    ui.text_edit_multiline(&mut self.new_task.text)
                });
            });
        }).response
    }
}
