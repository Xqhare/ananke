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
    tasks_list: List,
    tasks_vec: Vec<Task>,
    completed_vec : Vec<bool>,
    priority_vec: Vec<Option<String>>,
    complete_date_vec: Vec<Option<String>>,
    create_date_vec: Vec<Option<String>>,
    task_text: Vec<String>,
    project_tags_vec: Vec<Option<Vec<String>>>,
    context_tags_vec: Vec<Option<Vec<String>>>,
    special_tags_vec: Vec<Option<Vec<String>>>,
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
        let mut completed: Vec<bool> = Vec::new();
        let mut priority: Vec<Option<String>> = Vec::new();
        let mut complete_date: Vec<Option<String>> = Vec::new();
        let mut creation_date: Vec<Option<String>> = Vec::new();
        let mut task_str_out: Vec<String> = Vec::new();
        let mut project_tags: Vec<Option<Vec<String>>> = Vec::new();
        let mut context_tags: Vec<Option<Vec<String>>> = Vec::new();
        let mut special_tags: Vec<Option<Vec<String>>> = Vec::new();
        if let Ok(lines) = file_lines {
            
            for line in lines {
                if let Ok(task) = line {
                    // Setting up individual tasks for interrigation
                    let made_task: Task = Task::new(task);
                    // Extracting gui state from data
                    // Extracting completion status
                    let completed_out = match made_task.completed {
                        Some(value) => match value {
                            true => true,
                            _ => false,
                        },
                        _ => false,
                    };
                    completed.push(completed_out);
                    // Extracting priority
                    let priority_out = match made_task.priority {
                        Some(ref prio) => Option::from(prio.clone()),
                        _ => Option::default(),
                    };
                    priority.push(priority_out);
                    // Extracting completion date
                    let completion_out = match made_task.complete_date {
                        Some(ref date) => Option::from(date.clone()),
                        _ => Option::default(),
                    };
                    complete_date.push(completion_out);
                    // Extracting creation date
                    let creation_out = match made_task.create_date {
                        Some(ref date) => Option::from(date.clone()),
                        _ => Option::default(),
                    };
                    creation_date.push(creation_out);
                    // Extracting main text
                    task_str_out.push(made_task.task.clone());
                    // Extracting project tags
                    let project_out = match made_task.project_tags {
                        Some(ref tags) => Option::from(tags.clone()),
                        _ => Option::default(),
                    };
                    project_tags.push(project_out);
                    // Extracting context tags
                    let context_out = match made_task.context_tags {
                        Some(ref tags) => Option::from(tags.clone()),
                        _ => Option::default(),
                    };
                    context_tags.push(context_out);
                    // Extracting special tags
                    let special_out = match made_task.special_tags {
                        Some(ref tags) => Option::from(tags.clone()),
                        _ => Option::default(),
                    };
                    special_tags.push(special_out);
                    // pushing interrogated Task out
                    output.push(made_task.clone());
                }
            }
        
        }
        return TaskWidget{ tasks_list: todo_list, tasks_vec: output, completed_vec: completed, priority_vec: priority, complete_date_vec: complete_date, create_date_vec:creation_date, task_text: task_str_out, project_tags_vec: project_tags, context_tags_vec: context_tags, special_tags_vec: special_tags };
    }
    
}

impl TaskWidget {
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
    fn task_panel(&mut self, ctx: &eframe::egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading(format!("{NAME}"));
            ui.label(format!("by {AUTHOR}, v. {VERSION}"));
            let mut test_counter = 0;
            for entry in &self.tasks_vec {
                ui.add_space(PADDING);
                ui.separator();
                ui.horizontal(|ui| {
                    ui.separator();

                    let text = "Completed";
                    // The to be changed struct member HAS TO BE INSIDE the ui call! Got it!
                    ui.checkbox(&mut self.completed_vec[test_counter], text);
                    // Priority implementation
                    ui.text_edit_singleline(&mut match self.priority_vec[test_counter] {
                        Some(ref prio) => prio.clone(),
                        _ => String::new(),
                    });
                    // CODE BELOW WORKS!!!!!!
                    // --> Accessing the member of thhe vec in the Widget struct explicity
                    ui.text_edit_singleline(&mut self.task_text[test_counter]); 
                    let tester = &self.task_text[test_counter];
                    println!("test {tester}");
                    test_counter += 1;
                    });
            }
        });
    }
}

impl App for TaskWidget {
    // THIS IS THE MAIN LOOP
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.task_panel(ctx);
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
