use std::io;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use eframe::{run_native, App, egui::{CentralPanel, Ui}, NativeOptions};

use crate::{list::List, task::Task};

const PADDING: f32 = 4.0;
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

// TaskWidget is really the App itself
struct TaskWidget {
    tasks: List,
    test_tasks: Vec<Task>,
    test_strings: Vec<String>,
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
// this. -> How do I set the application state?
// => As I understand only in the TaskWidget struct, where I could store a Vec of tasks... if I'm
// right that is.
    


impl Default for TaskWidget {
    fn default() -> Self {
        let path: &str = "./todo-test.txt";
        let todo_list = List::open(path);
        let file_lines = Self::read_lines(path);
        let mut output: Vec<Task> = Vec::new();
        let mut test_str_out: Vec<String> = Vec::new();
        if let Ok(lines) = file_lines {
            
            for line in lines {
                if let Ok(task) = line {
                    let made_task: Task = Task::new(task);
                    test_str_out.push(made_task.task.clone());
                    output.push(made_task);
                }
            }
        
        }
        return TaskWidget{ tasks: todo_list, test_tasks: output, test_strings: test_str_out};
    }
    
}

impl TaskWidget {
    fn testing(&self, ctx: &eframe::egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(format!("{NAME}"));
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
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
    fn test_impl(&mut self, ctx: &eframe::egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(format!("{NAME}"));
            ui.label(format!("by {AUTHOR}, v. {VERSION}"));
            let mut test_counter = 0;
            for entry in &self.test_tasks {
                ui.add_space(PADDING);
                ui.separator();
                ui.horizontal(|ui| {
                    
                    ui.separator();
                    let mut checked = match entry.completed {
                        Some(answer) => match answer {
                            true => true,
                            _ => false,
                        }
                        _ => false,
                    };
                    let text = "Completed";
                    ui.checkbox(&mut checked, text);
                    let mut task_text: &str = entry.task.as_str();
                    ui.text_edit_singleline(&mut task_text);
                    // CODE BELOW WORKS!!!!!!
                    // --> Accessing the member of thhe vec in the Widget struct explicity
                    ui.text_edit_singleline(&mut self.test_strings[test_counter]); 
                    println!("test {task_text}");
                    test_counter += 1;
                    });
            }
        });
    }
}

impl App for TaskWidget {
    // THIS IS THE MAIN LOOP
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.test_impl(ctx);
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
