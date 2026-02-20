use anansi::Task;
use eframe::egui::Ui;
use horae::Utc;

use super::Ananke;

impl Ananke {
    pub fn task_list(&mut self, ui: &mut Ui) {
        for mut task in self.display_list.tasks() {
            ui.add_sized((ui.available_width(), 0.0), |ui: &mut Ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        self.is_done(ui, &mut task); 
                    });
                    ui.label(format!("{:?}", task));
                }).response
            });
        }
    }

    fn is_done(&mut self, ui: &mut Ui, task: &mut Task) {
        if ui.checkbox(task.mut_status(), "Done").clicked() {
            if task.is_done() {
                
            }
            match self.entire_list.update(task.clone(), task.id()) {
                Ok(_) => {
                    match self.display_list.update(task.clone(), task.id()) {
                        Ok(_) => (),
                        Err(e) => self.state.error = Some(e.into()),
                    }
                }
                Err(e) => {
                    self.state.error = Some(e.into());
                }
            }
        }
    }
}
