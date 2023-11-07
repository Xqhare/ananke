use std::io;
use chrono::{Utc, Datelike};
use unicode_segmentation::UnicodeSegmentation;
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead};
use std::fs::File;
use eframe::egui::{Grid, ScrollArea, Area};
use eframe::emath::Align2;
use eframe::epaint::Vec2;
use eframe::{run_native, App, egui::{CentralPanel, Ui}, NativeOptions};

use crate::{check_for_persistant_appstate, create_persistant_appstate};
use crate::task::{TaskDecoder, TaskEncoder};

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
    /// The file path to be read and saved to.
    file_path: PathBuf,
    /// Today's date formatted to YYYY-MM-DD.
    date: String,
    /// Needed for new task generation. Holds the date.
    new_create_date_in: String,
    /// Needed for new task generation. Holds the priotity.
    new_priority_in: String,
    /// Needed for new task generation. Holds the task main text.
    new_task_text_in: String,
    /// Needed to save state of greyed out creation date during task generation. Default `false`.
    new_edit_ui_date: bool,
    /// Needed to save the to be deleted tasks, to be deleted at another point in the loop. Default
    /// bool `false` and an empty Vec that will contain the indices.
    delete_task_touple: (bool, Vec<usize>),
    /// Workaround to show different content, here the help and about text. Default `false`.
    show_main_panel_about_text: bool,
    /// Workaround to show different content, here the welcome panel. Defalut `true`.
    show_main_panel_welcome_text: bool,
    /// Workaround to show different content, here the main scrollable task panel. Default `true`.
    show_task_scroll_area: bool,
    /// Workaround to show that the window now accepts drag and drop files. Default `false`.
    show_file_drop_area: bool,
    /// Workaround to show that ananke needs to be restarted. Default `false`.
    show_restart_area: bool,
    /// Workaround to show task creation dialoge. Default `false`.
    show_main_task_creation_area: bool,
    /// Workaround to show task deletion dialoge. Default `false`.
    show_task_deletion_collum: bool,
    
}

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
        let tuple_out: (bool, Vec<usize>) = (false, Vec::new());
        let mut path_out: PathBuf = PathBuf::new();
        let mut output: Vec<TaskDecoder> = Vec::new();
        let mut completed: Vec<bool> = Vec::new();
        let mut priority: Vec<String> = Vec::new();
        let mut complete_date: Vec<String> = Vec::new();
        let mut creation_date: Vec<String> = Vec::new();
        let mut task_str_out: Vec<String> = Vec::new();
        let mut project_tags: Vec<String> = Vec::new();
        let mut context_tags: Vec<String> = Vec::new();
        let mut special_tags: Vec<String> = Vec::new();
        let empty_string: String = String::new();

        let now = Utc::now();
        let date_today = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());
        
        let appstate = check_for_persistant_appstate();
        let tester = Self::read_lines(appstate.1.clone());
        let mut out_test = PathBuf::new();
        if let Ok(lines) = tester {
            for thing in lines {
                if let Ok(path_as_string) = thing {
                    let out = PathBuf::from(path_as_string);
                    out_test.push(out);
                }
            }
        }
        if appstate.0 {
            path_out = out_test.clone();
            let file_lines = Self::read_lines(out_test);
            
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
                                    project_out.push_str(" ");
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
                                    context_out.push_str(" ");
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
                                    special_out.push_str(" ");
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
            }
            return TaskWidget{tasks_vec: output, completed_vec: completed, priority_vec: priority, complete_date_vec: complete_date, create_date_vec:creation_date, task_text: task_str_out, project_tags_vec: project_tags, context_tags_vec: context_tags, special_tags_vec: special_tags, date: date_today.clone(), file_path: path_out, new_create_date_in: date_today.clone(), new_priority_in: empty_string.clone(), new_task_text_in: empty_string.clone(), new_edit_ui_date: false, delete_task_touple: tuple_out, show_main_panel_about_text: false, show_main_panel_welcome_text: true, show_task_scroll_area: true, show_file_drop_area: false, show_restart_area: false, show_main_task_creation_area: false, show_task_deletion_collum: false,};
    }
    
}

/// This implementation of `TaskWidget` really is only for helper, support, breakup functions, or for
/// gui functions that cannot be implemented in the implementation of `egui::App` for `TaskWidget`.
impl TaskWidget {
    /// This support function updates the contents of `TaskWidget` to the one's at the supplied path.
    fn update_from_path(&mut self, path: PathBuf) {
        let path_out: PathBuf = path.clone();
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
                                project_out.push_str(" ");
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
                                context_out.push_str(" ");
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
                                special_out.push_str(" ");
                            }
                        },
                        _ => special_out.push_str(""),
                    };
                    special_tags.push(special_out);
                    // pushing interrogated Task out
                    output.push(made_task.clone());
                }
            }
        self.file_path = path_out;
        self.completed_vec = completed;
        self.create_date_vec = creation_date;
        self.complete_date_vec = complete_date;
        self.priority_vec = priority;
        self.task_text = task_str_out;
        self.project_tags_vec = project_tags;
        self.context_tags_vec = context_tags;
        self.special_tags_vec = special_tags;
        }
    }
    /// This helper function reads a file by line from a supplied path (could be an &str of the absolute or relative path for examle).
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
    /// This gui function  creates the main window with the title, author, version. 
    fn main_panel(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        let temp = TaskEncoder::encode_taskwidget(self.clone());
                        let path: PathBuf = self.file_path.clone();
                        if TaskEncoder::save(temp, path).is_ok() {
                        }
                    }
                    if ui.button("Choose file location").clicked() {
                        if self.show_file_drop_area {
                            self.show_file_drop_area = false;
                        } else {
                            self.show_file_drop_area = true;
                        }
                    }
                });
                ui.menu_button("Task", |ui| {
                    if ui.button("New").clicked() {
                        if self.show_main_panel_about_text || self.show_main_panel_welcome_text {
                            self.show_main_panel_welcome_text = false;
                            self.show_main_panel_about_text = false;
                        }
                        self.show_main_task_creation_area = true;
                    }
                    if ui.button("Delete").clicked() {
                        if self.show_task_deletion_collum {
                            self.show_task_deletion_collum = false;
                        } else {
                            self.show_task_deletion_collum = true;
                            println!("DELETE TASK");
                        }
                    }
                    if ui.button("Change position").clicked() {
                        println!("CHANGE POS");
                    }
                });
                ui.menu_button("Sort", |ui| {
                    // This sorts by completion status and date
                    if ui.button("By completion and completion date").clicked() {
                        println!("SORT COMPLETION");
                    }
                    // sort by creation date
                    if ui.button("By inception date").clicked() {
                        println!("SORT CREATION DATE");
                    }
                    if ui.button("By priority").clicked() {
                        println!("SORT PRIO");
                    }
                    if ui.button("By project tags").clicked() {
                        println!("SORT PROJECT");
                    }
                    if ui.button("By context tags").clicked() {
                        println!("SORT CONTEXT");
                    }
                    if ui.button("By special tags").clicked() {
                        println!("SORT SPECIAL")
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                    // The most I have learned about buttons so far.
                    let test_button = ui.button("About");
                    if test_button.clicked() {
                        // The switch of welcome window is here to reduce lag / flickering in
                    // rendering
                    if self.show_main_panel_about_text == false {
                            self.show_main_panel_about_text = true;
                            self.show_main_panel_welcome_text = false;
                        } else {
                            self.show_main_panel_about_text = false;
                            self.show_main_panel_welcome_text = true;
                        };
                    }
                    // Legacy, here to remind myself of how it could be done.
                    if test_button.secondary_clicked() {
                        self.show_main_panel_about_text = false;
                    }
                });
                // Reset UI toggle
                if !self.show_task_scroll_area || !self.show_main_panel_welcome_text || self.show_task_deletion_collum || self.show_restart_area || self.show_file_drop_area {
                    if ui.button("Reset UI").clicked() {
                        self.show_task_deletion_collum = false;
                        self.show_main_panel_about_text = false;
                        self.show_file_drop_area = false;
                        self.show_restart_area = false;
                        self.show_main_task_creation_area = false;
                        // Default true:
                        self.show_main_panel_welcome_text = true;
                        self.show_task_scroll_area = true;
                    }
                }
                
                
            });
            let ui_main_area = ui.separator();
            let appstate_answer = check_for_persistant_appstate();
            if self.task_text.is_empty() {
                if appstate_answer.0 {
                    // read appstate and update self
                    self.show_file_drop_area = false;
                } else {
                    self.show_file_drop_area = true;
                }
            }
            if self.show_file_drop_area {
                Area::new("Drop todo.txt below:").anchor(Align2::CENTER_TOP, Vec2::from([0.0, 40.0])).show(ctx, |ui: &mut Ui| {
                    ui.heading("Drop file anywhere in this window!");
                    ctx.input(|i| {
                        if !i.raw.dropped_files.is_empty() {
                            for thing in &i.raw.dropped_files {
                                if thing.path.clone().is_some() {
                                    self.file_path = thing.path.clone().expect("No path!");
                                    self.show_file_drop_area = false;
                                    // I need a function to take in a pathbuf, save it
                                    // permanently, and then update self.
                                    create_persistant_appstate(appstate_answer.1.clone(), thing.path.clone().expect("No Path!"));
                                    self.update_from_path(thing.path.clone().expect("No Path!"));
                                }
                                self.show_restart_area = true;
                            }
                        }
                    });
                });
            }
            if self.show_restart_area {
                Area::new("Restart").anchor(Align2::CENTER_TOP, Vec2::from([0.0, 40.0])).show(ctx, |ui: &mut Ui| {
                    ui.heading("Please restart Ananke!");
                });
            }
            if self.show_main_task_creation_area {
                Grid::new(ui_main_area.id).show(ui, |ui: &mut Ui| {
                    let vec_strings = vec!["Create new task".to_string(), "Inception  date".to_string(), "Priority".to_string(), "Task".to_string()];
                    // Drawing the collum names
                    for mut name in vec_strings {
                        if name.contains("Create new task") {
                            let padded_name = Self::left_and_rightpad(25, name.clone());
                            name = padded_name;
                        }
                        if name.contains("Inception  date") {
                            let padded_name = Self::left_and_rightpad(6, name.clone());
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
                    ui.vertical(|ui: &mut Ui|{
                        ui.label("1. Delete inception date if it is unwanted by clicking the 'edit' button.");
                        ui.label("2. Set priority (any letter A-Z) if any is wanted. A being the highest, Z the lowest, proirity.");
                        ui.label("3. Enter task complete with +ProjectTags, @ContextTags and special:tags.");
                        ui.label("4. Hit save.");
                    });
                    // How to disable something (make it greyed out)
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.small_button("Edit").clicked() {
                            if self.new_edit_ui_date {
                                self.new_edit_ui_date = false;
                            } else {
                                self.new_edit_ui_date = true;
                            }
                            }
                            ui.add_enabled_ui(self.new_edit_ui_date, |ui| {
                                ui.text_edit_singleline(&mut self.new_create_date_in);
                            });
                        });
                        ui.horizontal(|ui: &mut Ui|{
                            ui.text_edit_singleline(&mut self.new_priority_in);
                        });
                        ui.text_edit_multiline(&mut self.new_task_text_in);
                        // Saving logic start:
                        if ui.button("Save").clicked() {
                            // Compile user input into todo.txt formatted string,
                            // function to decode the string and prepend it to TaskWidget elements
                            let mut encoded_out = String::new();
                            if self.new_create_date_in.graphemes(true).count() > 0 {
                                let out = format!("{} ", self.new_create_date_in);
                                encoded_out.push_str(&out);
                            } else {
                                encoded_out.push_str("");
                            }
                            if self.new_priority_in.graphemes(true).count() == 1 {
                                let out = format!("({}) ", self.new_priority_in);
                                encoded_out.push_str(&out);
                            } else if self.new_priority_in.graphemes(true).count() > 1 {
                                let out = format!("({}) ", self.new_priority_in.graphemes(true).take(1).last().unwrap());
                                encoded_out.push_str(&out);
                            } else {
                                encoded_out.push_str("");
                            }
                            encoded_out.push_str(self.new_task_text_in.as_str());
                            // Decoding created String
                            let decoded_task = TaskDecoder::new(encoded_out);
                            // As there is only one task, no update loop needed.
                            let index: usize = 0;
                            // There is no completion date OR completion marker!
                            // So first we push the non changing fields:
                            self.tasks_vec.insert(index, decoded_task.clone());
                            self.completed_vec.insert(index, false);
                            self.complete_date_vec.insert(index, String::new());
                            match decoded_task.create_date {
                                Some(date) => self.create_date_vec.insert(index, date),
                                None => self.create_date_vec.insert(index, String::new()),
                            }
                            match decoded_task.priority {
                                Some(prio) => self.priority_vec.insert(index, prio),
                                None => self.priority_vec.insert(index, String::new()),
                            }
                            // task is always some value!
                            self.task_text.insert(index, decoded_task.task);
                            // Da tags!
                            match decoded_task.project_tags {
                                Some(tag) => {
                                    let mut out = String::new();
                                    for entry in tag {
                                        out.push_str(&entry);
                                    }
                                    self.project_tags_vec.insert(index, out);
                                },
                                None => self.project_tags_vec.insert(index, String::new()),
                            }
                            match decoded_task.context_tags {
                                Some(tag) => {
                                    let mut out = String::new();
                                    for entry in tag {
                                        out.push_str(&entry);
                                    }
                                    self.context_tags_vec.insert(index, out);
                                },
                                None => self.context_tags_vec.insert(index, String::new()),
                            }
                            match decoded_task.special_tags {
                                Some(tag) => {
                                    let mut out = String::new();
                                    for entry in tag {
                                        out.push_str(&entry);
                                    }
                                    self.special_tags_vec.insert(index, out);
                                },
                                None => self.special_tags_vec.insert(index, String::new()),
                            }
                            // after saving logic, clear new task:
                            self.new_create_date_in = self.date.clone();
                            self.new_edit_ui_date = false;
                            self.new_priority_in = String::new();
                            self.new_task_text_in = String::new();
                        }
                    });
                    ui.end_row();
            }
            if self.show_task_scroll_area {
                ScrollArea::vertical().show(ui, |ui| {
                    if self.show_main_panel_welcome_text {
                        ui.heading(format!("Ananke - todo.txt editor"));
                        ui.label(format!("by {AUTHOR}, v. {VERSION}"));
                        ui.hyperlink_to(format!("{NAME} on github"), "https://github.com/Xqhare/ananke");
                    }
                    if self.show_main_panel_about_text {
                        ui.heading("About Ananke");
                        ui.label("Ananke is a fully-featured, end-to-end, zero-to-one Todo app that leverages the power of the todo.txt format to provide a seamless, frictionless and streamlined user experience.
    Built on a solid foundation of cutting-edge technologies, rust.");
                        ui.label("Ananke decodes your todo.txt, makes it look pretty and searchable, as well as creates new tasks, and updates finished ones.");
                        ui.heading("About the format todo.txt");
                        ui.label("The todo.txt format is a plain text format file for managing tasks. It is at it's core really only a .txt file named todo. It contains one task per line, and each task line can contain infomation like: A priority letter (A-Z) first, then the Inception (Creation) and Completion dates in (YYYY-MM-DD format), Project Tags (preceeded by the + sign), Context Tags (preceeded by the @ sign), and finally Special tags that only follow the [keyTag:AnyContentYouWantToBeSearchableWithTheKeyTag].");
                        ui.heading("Licenses");
                        ui.hyperlink_to(format!("egui licensed under the MIT-License"), "https://github.com/emilk/egui/blob/master/LICENSE-MIT");
                    }
                    if self.show_main_task_creation_area {
                    }
                    let mut counter = 0;
                    let vec_strings = vec!["#".to_string(), "Completed".to_string(), "Completion date".to_string(), "Inception date ".to_string(), "Priority".to_string(), "Task".to_string(), "Project  Tags".to_string(), "Context  Tags".to_string(), "Special  Tags".to_string()];
                    let task_list_seperator = ui.separator();
                    let _a_grid = Grid::new(task_list_seperator.id).striped(true).show(ui, |ui| {
                        // Drawing the collum names
                        for mut name in vec_strings {
                            if name.contains("Completed") && self.show_task_deletion_collum {
                                name = "Delete".to_string();
                            }
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
                        let mut delete_entry = false;
                        let mut delete_pos: Vec<usize> = Vec::new();
                        for _entry in &self.tasks_vec {
                            ui.label(counter.to_string());
                            let text = "Done!";
                            // The to be changed struct member HAS TO BE INSIDE the ui call! Got it!
                            // If task is marked as completed AND has a a creation date set, we set
                            // a completion date.
                            if self.show_task_deletion_collum {
                                if ui.button("Delete").clicked() {
                                    // Delete entry at counter!
                                    delete_entry = true;
                                    delete_pos.push(counter);
                                }
                            } else {
                                if ui.checkbox(&mut self.completed_vec[counter], text).clicked() {
                                    if self.completed_vec[counter] {
                                        if !self.create_date_vec[counter].is_empty() {
                                            let date_today = self.date.clone();
                                            self.complete_date_vec.remove(counter);
                                            self.complete_date_vec.insert(counter, date_today);
                                        }
                                    } else {
                                        self.complete_date_vec.remove(counter);
                                        self.complete_date_vec.insert(counter, String::new());
                                    }
                                }
                            }
                            
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
                        self.delete_task_touple = (delete_entry, delete_pos);
                    });
            });
            };
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
    /// It should be thought of as the rectangle that the app renders in.
    /// It takes over after being indirectly called in `gui.rs::main()`.
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.main_panel(ctx, frame);
        if self.delete_task_touple.0 {
            for pos in &mut self.delete_task_touple.1 {
                let position = pos.to_owned();
                self.tasks_vec.remove(position);
                self.completed_vec.remove(position);
                self.priority_vec.remove(position);
                self.complete_date_vec.remove(position);
                self.create_date_vec.remove(position);
                self.task_text.remove(position);
                self.project_tags_vec.remove(position);
                self.context_tags_vec.remove(position);
                self.special_tags_vec.remove(position);
            }
        }
    }
    
}

/// The main function should be thought of as the startup function, only defining the `app_name`
/// and the `NativeOptions` needed for running, and passing them on into `egui::run_native()`.
/// From here `update()` from `impl App for TaskWidget`
pub fn main() {
    let app_name = "Ananke";
    let size: Vec2<> = Vec2::from((1100.0, 800.0));
    let mut native_options = NativeOptions::default();
    {
        native_options.min_window_size = Option::from(size);
        native_options.drag_and_drop_support = true;
    }
    // the _cc is incredibly important, I don't know why
    run_native(app_name, native_options, Box::new(|_cc| {
        Box::<TaskWidget>::default()
    })).expect("E 001");
}

