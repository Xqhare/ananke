use eframe::egui::CentralPanel;

use crate::{error::AnankeError, gui::Ananke};


impl Ananke {
    pub fn load_file(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        for file in i.raw.dropped_files.iter() {
                            let path = file.path.clone();
                            if path.is_none() {
                                self.state.error = Some(AnankeError::new(
                                    "Error loading file",
                                    "Unable to read file path from file",
                                    None,
                                ));
                                break;
                            } else {
                                let tmp = path.unwrap();
                                let path_str = tmp.to_str();
                                if path_str.is_none() {
                                    self.state.error = Some(AnankeError::new(
                                        "Error loading file",
                                        "Unable to read file path from file",
                                        Some("Path is not valid unicode"),
                                    ));
                                    break;
                                } else {
                                    self.state.persistent_state.todo_file_path = path_str.unwrap().to_string();
                                    self.load_file = false;
                                }
                            }
                        }
                    }
                });
                ui.add_space(ui.available_height() / 2.5);
                ui.heading("Load file");
                ui.label("Drag and Drop a file!");
            });
        });

    }
}
