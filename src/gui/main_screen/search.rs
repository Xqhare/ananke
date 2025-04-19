use eframe::egui::{ComboBox, Response, Sides, Ui};

use crate::{gui::Ananke, state::{Show, SortBy}};

use super::editor::PRIOS;

impl Ananke {
    pub fn search(&mut self, ui: &mut eframe::egui::Ui) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Show:");
                    ComboBox::from_id_salt("Show").selected_text(format!("{}", self.state.search_state.show)).show_ui(ui, |ui: &mut Ui| {
                        for show in [Show::All, Show::Done, Show::Open] {
                            if ui.selectable_value(&mut self.state.search_state.show, show, format!("{show}")).clicked() {
                                self.state.search_state.show = show;
                            }
                        }
                    });
                    ui.label("Sort by:");
                    ComboBox::from_id_salt("Sort by").selected_text(format!("{}", self.state.search_state.sort_by)).show_ui(ui, |ui: &mut Ui| {
                        for sort_by in [SortBy::InceptionDate, SortBy::CompletionDate, SortBy::Priority] {
                            if ui.selectable_value(&mut self.state.search_state.sort_by, sort_by, format!("{sort_by}")).clicked() {
                                self.state.search_state.sort_by = sort_by;
                            }
                        }
                    });
                    ui.label("With priority:");
                    ComboBox::from_id_salt("Priority").selected_text(format!("{}", self.state.search_state.search_priority)).show_ui(ui, |ui: &mut Ui| {
                        for prio in PRIOS {
                            if ui.selectable_value(&mut self.state.search_state.search_priority, prio.to_string(), prio.to_string()).clicked() {
                                self.state.search_state.search_priority = prio.to_string();
                            }
                        }
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Search text:");
                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                        ui.text_edit_singleline(&mut self.state.search_state.search_text)
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Search project tags:");
                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                        ui.text_edit_singleline(&mut self.state.search_state.search_project)
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Search context tags:");
                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                        ui.text_edit_singleline(&mut self.state.search_state.search_context)
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Search special tags:");
                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                        ui.text_edit_singleline(&mut self.state.search_state.search_special)
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                    };
                    if ui.button("Search").clicked() {
                    };
                });
            });
        }).response
    }
}

