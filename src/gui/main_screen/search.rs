use eframe::egui::{ComboBox, Response, Ui};

use crate::{gui::Ananke, state::{Show, SortBy}};

impl Ananke {
    pub fn search(&mut self, ui: &mut eframe::egui::Ui) -> Response {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Show:");
                    ComboBox::from_id_salt("Show").selected_text(format!("{}", self.state.search_state.show)).show_ui(ui, |ui: &mut Ui| {
                        for show in [Show::All, Show::Done, Show::Open] {
                            if ui.selectable_value(&mut self.state.search_state.show, show, format!("{show}")).clicked() {
                            }
                        }
                    });
                    ui.label("Sort by:");
                    ComboBox::from_id_salt("Sort by").selected_text(format!("{}", self.state.search_state.sort_by)).show_ui(ui, |ui: &mut Ui| {
                        for sort_by in [SortBy::InceptionDate, SortBy::CompletionDate, SortBy::Priority] {
                            if ui.selectable_value(&mut self.state.search_state.sort_by, sort_by, format!("{sort_by}")).clicked() {
                            }
                        }
                    });
                    ui.separator();
                    if ui.button("Reset").clicked() {
                    };
                    if ui.button("Search").clicked() {
                    };
                });
                ui.horizontal(|ui| {
                });
            });
        }).response
    }
}

