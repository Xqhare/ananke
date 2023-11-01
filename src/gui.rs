use std::io;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use eframe::egui::{Grid, ScrollArea, Frame, Window, Area};
use eframe::emath::Align2;
use eframe::epaint::Vec2;
use eframe::{run_native, App, egui::{CentralPanel, Ui}, NativeOptions};

use crate::task::{TaskDecoder, self};

/// The author of the package.
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
/// The version of the package.
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// The name of the package.
const NAME: &str = env!("CARGO_PKG_NAME");

// TaskWidget is really the App itself

/// `TaskWidget` contains the App state, and can be thought of like the root of the entire App.
#[derive(Clone)]
pub struct TaskWidget {
    /// A vector of `Task`, primarily used for itteration. May be removed in the future.
    pub tasks_vec: Vec<TaskDecoder>,
    /// Vector containing the completed state of every task in order.
    pub completed_vec : Vec<bool>,
    /// Vector containing the priority state of every task in order, containing a empty String in
    /// case there isn't a priority state.
    pub priority_vec: Vec<String>,
    /// Vector containing the completion date of every task in order, containing a empty String in
    /// case there isn't a completion date.
    pub complete_date_vec: Vec<String>,
    /// Vector containing the creation date of every task in order, containing a empty String in
    /// case there isn't a creation date.
    pub create_date_vec: Vec<String>,
    /// Vector containing the main text of every task in order. As this is the minimum of required content of
    /// a task in the todo.txt format, there will never be an empty string inside.
    pub task_text: Vec<String>,
    /// Vector containing the vector of project tags, of every task in order, containing a empty String in
    /// case there aren't any project tags.
    pub project_tags_vec: Vec<String>,
    /// Vector containing the vector of context tags, of every task in order, containing a empty String in
    /// case there aren't any context tags.
    pub context_tags_vec: Vec<String>,
    /// Vector containing the vector of special tags, of every task in order, containing a empty String in
    /// case there aren't any context tags.
    pub special_tags_vec: Vec<String>,
    /// Workaround to show different content in window
    pub about_window: bool,
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
        let mut output: Vec<TaskDecoder> = Vec::new();
        let mut completed: Vec<bool> = Vec::new();
        let mut priority: Vec<String> = Vec::new();
        let mut complete_date: Vec<String> = Vec::new();
        let mut creation_date: Vec<String> = Vec::new();
        let mut task_str_out: Vec<String> = Vec::new();
        let mut project_tags: Vec<String> = Vec::new();
        let mut context_tags: Vec<String> = Vec::new();
        let mut special_tags: Vec<String> = Vec::new();
        if let Ok(lines) = file_lines {
            
            for line in lines {
                if let Ok(task) = line {
                    // Setting up individual tasks for interrigation
                    let made_task: TaskDecoder = TaskDecoder::new(task);
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
                        Some(ref date) => date.clone(),
                        _ => String::new(),
                    };
                    complete_date.push(completion_out);
                    // Extracting creation date
                    let creation_out = match made_task.create_date {
                        Some(ref date) => date.clone(),
                        _ => String::new(),
                    };
                    creation_date.push(creation_out);
                    // Extracting main text
                    task_str_out.push(made_task.task.clone());
                    // Extracting project tags
                    let mut project_out: String = String::new();
                    match made_task.project_tags {
                        Some(ref tags) => {
                            for tag in tags {
                                project_out.push_str(&tag);
                            }
                        },
                        _ => project_out.push_str(""),
                    };
                    project_tags.push(project_out);
                    // Extracting context tags
                    let mut context_out = String::new();
                    match made_task.context_tags {
                        Some(ref tags) => {
                            for tag in tags {
                                context_out.push_str(&tag);
                            }
                        },
                        _ => context_out.push_str(""),
                    };
                    context_tags.push(context_out);
                    // Extracting special tags
                    let mut special_out = String::new();
                    match made_task.special_tags {
                        Some(ref tags) => {
                            for tag in tags {
                                special_out.push_str(&tag);
                            }
                        },
                        _ => special_out.push_str(""),
                    };
                    special_tags.push(special_out);
                    // pushing interrogated Task out
                    output.push(made_task.clone());
                }
            }
        
        }
        let workaround = false;
        return TaskWidget{tasks_vec: output, completed_vec: completed, priority_vec: priority, complete_date_vec: complete_date, create_date_vec:creation_date, task_text: task_str_out, project_tags_vec: project_tags, context_tags_vec: context_tags, special_tags_vec: special_tags, about_window: workaround };
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
    fn tester(ctx: &eframe::egui::Context) {
        Area::new("testarea").show(ctx, |ui| {
            ui.label("whooo");
        });
    }
    /// This gui function creates the main window with the title, author, version
    fn task_panel(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        let _temp = task::TaskEncoder::encode_taskwidget(self.clone());
                    };
                    if ui.button("Choose file location").clicked() {
                        println!("FILE LOCATION!");
                    }
                });
                ui.menu_button("Task", |ui| {
                    if ui.button("New").clicked() {
                        println!("NEW TASK");
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                    if ui.button("About").clicked() {
                        self.about_window = true;
                    }
                });
                
            });
            ui.separator();
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading(format!("Ananke - todo.txt editor"));
                ui.label(format!("by {AUTHOR}, v. {VERSION}"));
                ui.hyperlink_to(format!("{NAME} on github"), "https://github.com/Xqhare/ananke");
                
                let mut counter = 0;
                let vec_strings = vec!["Completed".to_string(), "Completion date".to_string(), "Inception date ".to_string(), "Priority".to_string(), "Task".to_string(), "Project  Tags".to_string(), "Context  Tags".to_string(), "Special  Tags".to_string()];
                let task_list_seperator = ui.separator();
                let _a_grid = Grid::new(task_list_seperator.id).striped(true).show(ui, |ui| {
                    // Drawing the collum names
                    for mut name in vec_strings {
                        // The 2 whitespace in the Project  Tags is on purpose! - As well
                        // as in the other Tags too!
                        if name.contains("Project  Tags") {
                            // 12 whitespace padding
                            let padded_name = Self::left_and_rightpad(12, name.clone());
                            name = padded_name;
                        }
                        if name.contains("Context  Tags") {
                            // 12 whitespace padding
                            let padded_name = Self::left_and_rightpad(12, name.clone());
                            name = padded_name;
                        }
                        if name.contains("Special  Tags") {
                            // 12 whitespace padding
                            let padded_name = Self::left_and_rightpad(12, name.clone());
                            name = padded_name;
                        }
                        if name.contains("Task") {
                            // 40 whitespace padding
                            let padded_name = Self::left_and_rightpad(40, name.clone());
                            name = padded_name;
                        }
                        ui.label(name);
                    }
                    ui.end_row();
                    for _entry in &self.tasks_vec {
                        let text = "Done!";
                        // The to be changed struct member HAS TO BE INSIDE the ui call! Got it!
                        ui.checkbox(&mut self.completed_vec[counter], text);
                        // completion and creation dates
                        ui.text_edit_singleline(&mut self.complete_date_vec[counter]);
                        ui.text_edit_singleline(&mut self.create_date_vec[counter]);
                        // Priority implementation
                        // variable input fields are very versitile!
                        ui.text_edit_singleline(&mut self.priority_vec[counter]);
                        ui.text_edit_multiline(&mut self.task_text[counter]);
                        // Da tags!!
                        ui.text_edit_multiline(&mut self.project_tags_vec[counter]);
                        ui.text_edit_multiline(&mut self.context_tags_vec[counter]);
                        ui.text_edit_multiline(&mut self.special_tags_vec[counter]);
                        // End of task; -> advance counter by one and end the row
                        counter += 1;
                        ui.end_row();
                    };
                });
                // There should be a save button at the top; Quit could be only handled
                // with window close?
                /* let _save_quit_bottom_seperator = ui.separator();
                ui.horizontal(|ui: &mut Ui| {
                    if ui.add(Button::new("Save test1").fill(Color32::from_rgb(6, 143, 15))).clicked() {
                        println!("SAVE OUT 1")
                    };

                    if ui.button("Save").clicked() {
                        println!("SAVE OUT");
                    }
                }); */
            });
        });
    }
    /// This helper function padds a String with `x` amount of whitespace.
    fn left_and_rightpad(padding: u16, input_string: String) -> String {
        let mut right_pad = String::new();
        for _ in 0..padding {
            right_pad.push_str(" ");
        }
        let left_pad = right_pad.clone();
        let mut output = input_string.clone();
        output.insert_str(0, &left_pad);
        output.push_str(&right_pad);
        return output;
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
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.task_panel(ctx, frame);
        if self.about_window {
            Self::tester(ctx)
        }
    }
    
}

/// The main function should be thought of as the startup function, only defining the `app_name`
/// and the `NativeOptions` needed for running, and passing them on into `egui::run_native()`.
/// From here `update()` from `impl App for TaskWidget`
pub fn main() {
    let app_name = "Ananke";
    let size: Vec2<> = Vec2::from((1050.0, 800.0));
    let mut native_options = NativeOptions::default();
    {
        native_options.min_window_size = Option::from(size);
    }
    // the _cc is incredibly important, I don't know why
    run_native(app_name, native_options, Box::new(|_cc| {
        Box::<TaskWidget>::default()
    })).expect("E 001");
}
