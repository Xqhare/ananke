use anansi::Task;
use eframe::egui::Ui;
use horae::Utc;


use crate::error::AnankeError;

use super::Ananke;

impl Ananke {
    pub fn task_list(&mut self, ui: &mut Ui) {
        for task in self.display_list.tasks() {
            ui.add_sized((ui.available_width(), 0.0), |ui: &mut Ui| {
                ui.group(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        self.is_done(ui, task.clone()); 
                    });
                    ui.label(format!("{:?}", task));
                }).response
            });
        }
    }

    fn is_done(&mut self, ui: &mut Ui, task: Task) {
        ui.horizontal(|ui| {
            
            if task.is_done() {
                ui.label("Status: Done!");
                if ui.button("Mark Open").clicked() {
                    let new_task = task.undone();
                    match self.entire_list.update(new_task.clone(), task.id()) {
                        Ok(_) => {
                            match self.display_list.update(new_task.clone(), task.id()) {
                                Ok(_) => {}
                                Err(e) => {
                                    self.state.error = Some(e.into());
                                }
                            }
                        }
                        Err(e) => {
                            self.state.error = Some(e.into());
                        }
                    }
                }
            } else {
                ui.label("Status: Open");
                if ui.button("Mark Completed").clicked() {
                    let mut now = Utc::now();
                    now.with_timezone(self.state.persistent_state.timezone);
                    let new_task = task.done(Some(now.date().to_string().into()));
                    match new_task {
                        Ok(new_task) => {
                            let id = new_task.id();
                            match self.entire_list.update(new_task.clone(), id) {
                                Ok(_) => {
                                    match self.display_list.update(new_task.clone(), id) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            self.state.error = Some(e.into());
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.state.error = Some(e.into());
                                }
                            }
                        }
                        Err(e) => {
                            self.state.error = Some(e.into());
                        }
                    }
                }
            }
        });
    }
}
