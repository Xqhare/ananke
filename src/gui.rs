use eframe::{run_native, App, egui::{CentralPanel, Ui}, NativeOptions};

use crate::list::List;

const PADDING: f32 = 4.0;
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
struct TaskWidget {
    tasks: List,
}

/* impl TaskWidget {
    fn new() -> TaskWidget {
        let path: &str = "./todo-test.txt";
        let todo_list: List = List::open(path);
        return TaskWidget{ tasks: todo_list};
    }
} */

// I need to be able to mutate the data in task.rs... I'm thinking about, to work around the
// truncation problem at saving, to delete and save new anyway, so more jank is no real deal
// breaker; BUT the only way I currently see is to clone the data inside task.rs, then display it,
// MAYBE I'll update the data in task to save from there? OR I'll just save the data from the GUI.
// I hate both approaches equally. The only way to make it not jank would have to start with a
// propper save implementation, that could be copied from sesaht.
// Making the data in tasks mutable however is proving to be tricky.
//
// Saving or copying everything into the TaskWidet struct seems to be a better idea, and more
// practical. Hopefully.
//
// I no longer think that it has to do with mutability; the gui is just reading it again and again?
// -> After a quick println debugging session: yes. It is constantly being read. How tf do i fix
// this.

impl Default for TaskWidget {
    fn default() -> Self {
        let path: &str = "./todo-test.txt";
        let todo_list = List::open(path);
        return TaskWidget{ tasks: todo_list};
    }
}

impl TaskWidget {
    fn testing(&self, ctx: &eframe::egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(format!("{NAME} "));
            ui.label(format!("by {AUTHOR}, v. {VERSION}"));
            for mut task in self.tasks.return_all_tasks() {
                ui.add_space(PADDING);
                ui.separator();
                ui.horizontal(|ui| {
                    
                    ui.separator();
                    let mut checked = match task.completed {
                        Some(answer) => match answer {
                            true => true,
                            _ => false,
                        }
                        _ => false,
                    };
                    let text = "Completed";
                    ui.checkbox(&mut checked, text);
                    let mut task_text: String = task.copy_task();
                    ui.text_edit_singleline(&mut task.copy_task());
                    println!("test {task_text}")
                    });
            }
            ui.separator();
        });
    }
}

impl App for TaskWidget {
    // THIS IS THE MAIN LOOP
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.testing(ctx);
    }
    
}

pub fn main() {
    let app_name = "Ananke";
    let native_options = NativeOptions::default();
    // the _cc is incredibly important, I don't know why
    run_native(app_name, native_options, Box::new(|_cc| {
        Box::<TaskWidget>::default()
    })).expect("E 001");
}
