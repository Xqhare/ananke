use std::io;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use eframe::egui::{Grid, ScrollArea, TopBottomPanel};
use eframe::{run_native, App, egui::{CentralPanel, Ui}, NativeOptions};

use crate::task::Task;

/// The author of the package.
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
/// The version of the package.
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The name of the package.
const NAME: &str = env!("CARGO_PKG_NAME");

// TaskWidget is really the App itself

/// `TaskWidget` contains the App state, and can be thought of like the root of the entire App.
struct TaskWidget {
    /// A vector of `Task`, primarily used for itteration. May be removed in the future.
    tasks_vec: Vec<Task>,
    /// Vector containing the completed state of every task in order.
    completed_vec : Vec<bool>,
    /// Vector containing the priority state of every task in order, containing a empty String in
    /// case there isn't a priority state.
    priority_vec: Vec<String>,
    /// Vector containing the completion date of every task in order, containing a empty String in
    /// case there isn't a completion date.
    complete_date_vec: Vec<Option<String>>,
    /// Vector containing the creation date of every task in order, containing a empty String in
    /// case there isn't a creation date.
    create_date_vec: Vec<Option<String>>,
    /// Vector containing the main text of every task in order. As this is the minimum of required content of
    /// a task in the todo.txt format, there will never be an empty string inside.
    task_text: Vec<String>,
    /// Vector containing the vector of project tags, of every task in order, containing a empty String in
    /// case there aren't any project tags.
    project_tags_vec: Vec<Option<Vec<String>>>,
    /// Vector containing the vector of context tags, of every task in order, containing a empty String in
    /// case there aren't any context tags.
    context_tags_vec: Vec<Option<Vec<String>>>,
    /// Vector containing the vector of special tags, of every task in order, containing a empty String in
    /// case there aren't any context tags.
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
    
// Soo egui reaaaly doesn't like options... wich isn't a problem now that i know what wreaked havok


/// Implementing the Default value for `TaskWidget`, interrogates the task returned from the decoding
/// steps in `task.rs`.
///
/// This is the only way of creating a `TaskWidget`.
impl Default for TaskWidget {
    /// This function creates the new `TaskWidget`.
    ///
    /// At the moment it has a hard coded location, that is then read by line.
    /// Each line is then interrogated and the appropriate response saved into the struct fields of
    /// `TaskWidget`.
    fn default() -> Self {
        let path: &str = "./todo-test.txt";
        let file_lines = Self::read_lines(path);
        let mut output: Vec<Task> = Vec::new();
        let mut completed: Vec<bool> = Vec::new();
        let mut priority: Vec<String> = Vec::new();
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
                        Some(ref prio) => prio.clone(),
                        _ => String::new(),
                    };
                    priority.push(priority_out);
                    // Extracting completion date
                    let completion_out = match made_task.complete_date {
                        Some(ref date) => Option::from(date.clone()),
                        _ => Option::from(String::from("")),
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
        return TaskWidget{tasks_vec: output, completed_vec: completed, priority_vec: priority, complete_date_vec: complete_date, create_date_vec:creation_date, task_text: task_str_out, project_tags_vec: project_tags, context_tags_vec: context_tags, special_tags_vec: special_tags };
    }
    
}

/// This implementation of `TaskWidget` really is only for helper, support, breakup functions, or for
/// gui functions that cannot be implemented in the implementation of `egui::App` for `TaskWidget`.
impl TaskWidget {
    /// This helper function reads a file by line from a supplied path (could be an &str of the absolute or relative path for examle).
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
    /// This gui function creates the main window with the title, author, version
    fn task_panel(&mut self, ctx: &eframe::egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading(format!("Ananke - todo.txt editor"));
                ui.label(format!("by {AUTHOR}, v. {VERSION}"));
                ui.hyperlink_to(format!("{NAME} on github"), "https://github.com/Xqhare/ananke");
                let mut counter = 0;
                let vec_strings = vec!["Completed".to_string(), "Priority".to_string(), "Task".to_string()];
                let temp = ui.separator();
                let _a_grid = Grid::new(temp.id).striped(true).show(ui, |ui| {
                    // Drawing the collum names
                    for mut name in vec_strings {
                        if name.contains("Task") {
                            // 40 whitespace padding - let's just call it 'oldschool' ok?
                            let right_pad = "                                       ";
                            let left_pad = right_pad;
                            name.insert_str(0, left_pad);
                            name.push_str(right_pad);
                        }
                        ui.label(name);
                    }
                    ui.end_row();
                    for _entry in &self.tasks_vec {
                        let text = "Done!";
                        // The to be changed struct member HAS TO BE INSIDE the ui call! Got it!
                        // workaround:
                        ui.checkbox(&mut self.completed_vec[counter], text);
                        // Priority implementation
                        // variable input fields are very versitile!
                        ui.text_edit_singleline(&mut self.priority_vec[counter]);
                        ui.text_edit_multiline(&mut self.task_text[counter]);
                        // CODE BELOW WORKS!!!!!!
                        // --> Accessing the member of thhe vec in the Widget struct explicity
                        // ui.text_edit_singleline(&mut self.task_text[counter]); 
                        // let tester = &self.task_text[counter];
                        // println!("test {tester}");
                        counter += 1;
                        ui.end_row();
                    };
                });
            });
        });
    }
}

/// This implementaion is the integration with `egui` and can only contain a limited number of
/// functions.
///
/// Most importantly it contains the main loop of the gui.
impl App for TaskWidget {
    // THIS IS THE MAIN LOOP
    /// This function is the main loop of ananke, being called as often as possible (60 times/sec I
    /// think).
    ///
    /// It takes over after being indirectly called in `gui.rs::main()`.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.task_panel(ctx);
    }
    
}

/// The main function should be thought of as the startup function, only defining the `app_name`
/// and the `NativeOptions` needed for running, and passing them on into `egui::run_native()`.
/// From here `update()` from `impl App for TaskWidget`
pub fn main() {
    let app_name = "Ananke";
    let native_options = NativeOptions::default();
    // the _cc is incredibly important, I don't know why
    run_native(app_name, native_options, Box::new(|_cc| {
        Box::<TaskWidget>::default()
    })).expect("E 001");
}
