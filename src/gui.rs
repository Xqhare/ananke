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
    /// A vector of `Task`, primarily used for itteration. May be removed in the future. Could
    /// be used for a reset task feature in the future.
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
    /// Needed for user input of new task position. Default `empty`.
    usr_change_pos_in: Vec<String>,
    /// Needed to save the to be moved tasks, to move them at another point in the loop. Default
    /// bool `false` and an empty `Vec` of touples of `usize` numbers.
    change_task_touple: (bool, Vec<(usize, usize)>),
    /// Saves indices of tasks to be displayed because of sorting. Default `empty`.
    sort_tasks_indices: Vec<usize>,
    /// Saves a vector of `String`'s that's used to search for it's contents, in this case the
    /// task text.
    sort_task_text: Vec<String>,
    /// Saves a vector of `String`'s that's used to search for it's contents, in this case the
    /// task text.
    sort_project_tags: Vec<String>,
    /// Saves a vector of `String`'s that's used to search for it's contents, in this case the
    /// task text.
    sort_context_tags: Vec<String>,
    /// Saves a vector of touple's of `String`'s that's used to search for it's contents, in this case the
    /// task text. The touple contains: leading the tag, and trailing the text.
    sort_special_tags: Vec<String>,
    /// Stores the user inputed special tags decoded, ready to search.
    sort_special_tags_decoded: Vec<(String, String)>,
    /// Stores the appstate special tags decoded, ready to be searched.
    sortable_special_tags: Vec<(String, String)>,
    /// Holds the user input to sort by completion. Default `false`.
    usr_sort_completion: bool,
    /// Holds the user input to sort by creation date. Default `false`.
    usr_sort_create_date: bool,
    /// Holds the user input to sort by priority. Default `false`.
    usr_sort_priority: bool,
    /// Saves the direct user input of the task, ready to be decoded. Default `Enter task text
    /// to search for`.
    usr_sort_task_text_in: String,
    /// Saves the direct user input of the task, ready to be decoded. Default `Enter
    /// +ProjectTags
    /// to search`.
    usr_sort_project_tags_in: String,
    /// Saves the direct user input of the task, ready to be decoded. Default `Enter
    /// @ContextTags
    /// to search`.
    usr_sort_context_tags_in: String,
    /// Saves the direct user input of the task, ready to be decoded. Default `Enter
    /// Special:Tags
    /// to search`.
    usr_sort_special_tags_in: String,
    /// Workaround to show different content, here the help and about text. Default `false`.
    show_main_panel_about_text: bool,
    /// Workaround to show different content, here the welcome panel. Defalut `true`.
    show_main_panel_welcome_text: bool,
    /// Workaround to show different content, here the main scrollable task panel. Default `true`.
    show_task_scroll_area: bool,
    /// Workaround to show that the window now accepts drag and drop files. Default `false`.
    show_file_drop_area: bool,
    /// Workaround to show task creation dialoge. Default `false`.
    show_main_task_creation_area: bool,
    /// Workaround to show task deletion dialoge. Default `false`.
    show_task_deletion_collum: bool,
    /// Workaround to show move task position dialoge. Default `false`.
    show_task_move_pos_collum: bool,
    /// Workaround to show the main sorting area. Default `false`.
    show_main_sorting_area: bool,
    /// Workaround to show the "No results found" dialoge after a failed search. Default `false`.
    show_no_results_found_text: bool,
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
        let change_touple: (bool, Vec<(usize, usize)>) = (false, Vec::new());
        let delete_touple: (bool, Vec<usize>) = (false, Vec::new());
        let special_tag_touple: Vec<(String, String)> = Vec::new();
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
        let mut empty_vec_string: Vec<String> = Vec::new();
        let mut special_tags_decoded: Vec<(String, String)> = Vec::new();
        let sorting_indices: Vec<usize> = Vec::new();

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
                        empty_vec_string.push(empty_string.clone());
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
                        let mut special_decoded_out = (String::new(), String::new());
                        match made_task.special_tags {
                            Some(ref tags) => {
                                for tag in tags {
                                    special_out.push_str(&tag);
                                    special_out.push_str(" ");
                                    let temp_val = tag.split_once(":");
                                    if temp_val.is_some() {
                                        special_decoded_out.0.push_str(temp_val.unwrap().0);
                                        special_decoded_out.1.push_str(temp_val.unwrap().1);
                                    }
                                }
                            },
                            _ => special_out.push_str(""),
                        };
                        special_tags.push(special_out);
                        special_tags_decoded.push(special_decoded_out);
                        // pushing interrogated Task out
                        output.push(made_task.clone());
                    }
                }
            }
            }
            return TaskWidget{tasks_vec: output, completed_vec: completed, priority_vec: priority, complete_date_vec: complete_date, create_date_vec:creation_date, task_text: task_str_out, project_tags_vec: project_tags, context_tags_vec: context_tags, special_tags_vec: special_tags, date: date_today.clone(), file_path: path_out, new_create_date_in: date_today.clone(), new_priority_in: empty_string.clone(), new_task_text_in: empty_string.clone(), new_edit_ui_date: false, delete_task_touple: delete_touple, usr_change_pos_in: empty_vec_string.clone(), change_task_touple: change_touple, show_main_panel_about_text: false, show_main_panel_welcome_text: true, show_task_scroll_area: true, show_file_drop_area: false, show_main_task_creation_area: false, show_task_deletion_collum: false, show_task_move_pos_collum: false, show_main_sorting_area: false, sort_task_text: empty_vec_string.clone(), sort_project_tags: empty_vec_string.clone(), sort_context_tags: empty_vec_string.clone(), sort_special_tags: empty_vec_string.clone(), usr_sort_task_text_in: "Enter task text to search".to_string(), usr_sort_project_tags_in: "Enter +ProjectTags to search".to_string(), usr_sort_context_tags_in: "Enter @ContextTags to search".to_string(), usr_sort_special_tags_in: "Enter Special:Tags to search".to_string(), usr_sort_completion: false, usr_sort_create_date: false, usr_sort_priority: false, sort_special_tags_decoded: special_tag_touple.clone(), sortable_special_tags: special_tags_decoded, sort_tasks_indices: sorting_indices, show_no_results_found_text: false, };
    }
    
}

/// This implementation of `TaskWidget` really is only for helper, support, breakup functions, or for
/// gui functions that cannot be implemented in the implementation of `egui::App` for `TaskWidget`.
impl TaskWidget {
    /// This helper function is called when the user has entered a `String` into the text
    /// search box. It reads out the input, decodes it and saves the indices of the hits. 
    ///
    /// ## Technical info
    /// I just read the entire task text and search it for the input;
    /// If the struct member `sort_tasks_indices` is filled, I truncate, as this search has
    /// priority over the booleans; They will be called anyway after, and handle the prefilled
    /// struct member already.
    fn sort_task_text(&mut self) {
        let mut output_vec: Vec<usize> = Vec::new();
        let mut counter: usize = 0;
        for task_text in self.task_text.clone() {
            if task_text.contains(&self.usr_sort_task_text_in) {
                output_vec.push(counter);
                println!("Debug {task_text}");
            }
            counter +=1;
        }
        self.sort_tasks_indices = output_vec;
    }
    /// This helper function, sorts all taskes by completion / creation date / priority.Any
    /// combination of the three is valid, with completion being always first, then creation
    /// date, then priority sorting.
    fn sort_true_false(&mut self) {
        let mut sorted_output: Vec<usize> = Vec::new();
        if self.sort_tasks_indices.len() > 0 {
            sorted_output = self.sort_tasks_indices.clone();
        }
        if self.usr_sort_priority {
            // If sorted_output is not empty
            if sorted_output.len() > 0 {
                // getting out all indices with creation dates.
                let mut temp_sorting_vec: Vec<(String, usize)> = Vec::new();
                let mut rest_indices: Vec<usize> = Vec::new();
                for index in sorted_output.clone() {
                    if self.priority_vec[index].len() > 0 {
                        let aa = (self.priority_vec[index].clone(), index);
                        temp_sorting_vec.push(aa);
                    } else {
                        rest_indices.push(index);
                    }
                }
                // sorting the indices
                temp_sorting_vec.sort();
                let mut out: Vec<usize> = Vec::new();
                for entry in temp_sorting_vec {
                    out.push(entry.1);
                }
                out.append(&mut rest_indices);
                sorted_output = out;
            } else {
                let mut counter: usize = 0;
                let mut temp_sorting: Vec<(String, usize)> = Vec::new();
                for task in self.priority_vec.clone() {
                    if task.len() > 0 {
                        temp_sorting.push((task, counter.clone()));
                    }
                    counter += 1;
                }
                temp_sorting.sort();
                let mut out: Vec<usize> = Vec::new();
                for entry in temp_sorting {
                    out.push(entry.1);
                }
                sorted_output = out;
            }
        }
        if self.usr_sort_create_date {
            // If sorted_output is not empty
            if sorted_output.len() > 0 {
                // getting out all indices with creation dates.
                let mut temp_sorting_vec: Vec<(String, usize)> = Vec::new();
                let mut rest_indices: Vec<usize> = Vec::new();
                for index in sorted_output.clone() {
                    if self.create_date_vec[index].len() > 0 {
                        let aa = (self.create_date_vec[index].clone(), index);
                        temp_sorting_vec.push(aa);
                    } else {
                        rest_indices.push(index);
                    }
                }
                // sorting the indices
                temp_sorting_vec.sort();
                let mut out: Vec<usize> = Vec::new();
                for entry in temp_sorting_vec {
                    out.push(entry.1);
                }
                out.append(&mut rest_indices);
                sorted_output = out;
            } else {
                let mut counter: usize = 0;
                let mut temp_sorting: Vec<(String, usize)> = Vec::new();
                for task in self.create_date_vec.clone() {
                    if task.len() > 0 {
                        temp_sorting.push((task, counter.clone()));
                    }
                    counter += 1;
                }
                temp_sorting.sort();
                let mut out: Vec<usize> = Vec::new();
                for entry in temp_sorting {
                    out.push(entry.1);
                }
                sorted_output = out;
            }
        }
        if self.usr_sort_completion {
            // If sorted_output is not empty
            if sorted_output.len() > 0 {
                // getting out all indices with creation dates.
                let mut temp_sorting_vec: Vec<(bool, usize)> = Vec::new();
                let mut rest_indices: Vec<usize> = Vec::new();
                for index in sorted_output.clone() {
                    if self.completed_vec[index] {
                        let aa = (self.completed_vec[index].clone(), index);
                        temp_sorting_vec.push(aa);
                    } else {
                        rest_indices.push(index);
                    }
                }
                // sorting the indices
                temp_sorting_vec.sort();
                let mut out: Vec<usize> = Vec::new();
                for entry in temp_sorting_vec {
                    out.push(entry.1);
                }
                out.append(&mut rest_indices);
                sorted_output = out;
            } else {
                let mut counter: usize = 0;
                let mut temp_sorting: Vec<(bool, usize)> = Vec::new();
                for task in self.completed_vec.clone() {
                    if task {
                        temp_sorting.push((task, counter.clone()));
                    }
                    counter += 1;
                }
                temp_sorting.sort();
                let mut out: Vec<usize> = Vec::new();
                for entry in temp_sorting {
                    out.push(entry.1);
                }
                sorted_output = out;
            }
        }
        self.sort_tasks_indices = sorted_output.clone();
    }
    fn reset_grid_ui(&mut self) {
        self.show_no_results_found_text = false;
        self.show_task_deletion_collum = false;
        self.show_file_drop_area = false;
        self.show_task_move_pos_collum = false;
        // Default true:
        self.show_task_scroll_area = true;
    }
    fn reset_top_ui(&mut self) {
        self.show_no_results_found_text = false;
        self.show_main_panel_about_text = false;
        self.show_file_drop_area = false;
        self.show_main_task_creation_area = false;
        self.show_main_sorting_area = false;
        // Default true:
        self.show_main_panel_welcome_text = true;
    }
    fn reset_all_ui(&mut self) {
        self.show_no_results_found_text = false;
        self.show_task_deletion_collum = false;
        self.show_main_panel_about_text = false;
        self.show_file_drop_area = false;
        self.show_main_task_creation_area = false;
        self.show_task_move_pos_collum = false;
        self.show_main_sorting_area = false;
        // Default true:
        self.show_main_panel_welcome_text = true;
        self.show_task_scroll_area = true;
        // Reset shown tasks
        self.sort_tasks_indices = Vec::new();
    }
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
        let mut special_tags_decoded: Vec<(String, String)> = Vec::new();
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
                    let mut special_decoded_out = (String::new(), String::new());
                    match made_task.special_tags {
                        Some(ref tags) => {
                            for tag in tags {
                                special_out.push_str(&tag);
                                special_out.push_str(" ");
                                let temp_val = tag.split_once(":");
                                if temp_val.is_some() {
                                    special_decoded_out.0.push_str(temp_val.unwrap().0);
                                    special_decoded_out.1.push_str(temp_val.unwrap().1);
                                }
                            }
                        },
                        _ => special_out.push_str(""),
                    };
                    special_tags.push(special_out);
                    special_tags_decoded.push(special_decoded_out);
                    // pushing interrogated Task out
                    output.push(made_task.clone());
                }
            }
        self.tasks_vec = output;
        self.file_path = path_out;
        self.completed_vec = completed;
        self.create_date_vec = creation_date;
        self.complete_date_vec = complete_date;
        self.priority_vec = priority;
        self.task_text = task_str_out;
        self.project_tags_vec = project_tags;
        self.context_tags_vec = context_tags;
        self.special_tags_vec = special_tags;
        self.sortable_special_tags = special_tags_decoded;
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
                        let _ = TaskEncoder::save(temp, path);
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
                        if !self.show_main_task_creation_area {
                            // I know this looks wierd, but it's simple, first the top ui
                            // is reset and THEN the defaults are disabled!
                            self.reset_top_ui();
                            if self.show_main_panel_about_text || self.show_main_panel_welcome_text {
                                self.show_main_panel_welcome_text = false;
                                self.show_main_panel_about_text = false;
                            }
                            self.reset_grid_ui();
                            self.show_main_task_creation_area = true;
                        } else {
                            self.reset_top_ui();
                        }
                        
                    }
                    if ui.button("Delete").clicked() {
                        if self.show_task_deletion_collum {
                            self.reset_grid_ui();
                        } else {
                            self.reset_grid_ui();
                            self.show_task_deletion_collum = true;
                        }
                    }
                    if ui.button("Change position").clicked() {
                        if !self.show_task_move_pos_collum {
                            self.reset_grid_ui();
                            self.show_task_move_pos_collum = true;
                        } else {
                            self.reset_grid_ui();
                        }
                    }
                });
                if ui.button("Search").clicked() {
                    if !self.show_main_sorting_area {
                        self.reset_top_ui();
                        self.reset_grid_ui();
                        self.show_main_panel_welcome_text = false;
                        self.show_main_sorting_area = true;
                    } else {
                        self.reset_top_ui();
                    }
                }
                ui.menu_button("Help", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                    // The most I have learned about buttons so far.
                    if ui.button("About").clicked() {
                        // The switch of welcome window is here to reduce lag / flickering in
                        // rendering
                        if self.show_main_panel_about_text == false {
                                self.reset_top_ui();
                                self.show_main_panel_about_text = true;
                                self.show_main_panel_welcome_text = false;
                            } else {
                                self.reset_top_ui();
                            };
                    }
                });
                // Reset UI toggle
                if !self.show_task_scroll_area || !self.show_main_panel_welcome_text || self.show_task_deletion_collum || self.show_file_drop_area || self.show_task_move_pos_collum {
                    if ui.button("Reset UI").clicked() {
                       self.reset_all_ui();
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
                                    // I need a function to take in a pathbuf, save it
                                    // permanently, and then update self.
                                    create_persistant_appstate(appstate_answer.1.clone(), thing.path.clone().expect("No Path!"));
                                    // this is called, but the output doesn't update... is self not
                                    // read again?
                                    self.update_from_path(thing.path.clone().expect("No Path!"));
                                    self.show_file_drop_area = false;
                                }
                            }
                        }
                    });
                });
            }
            // Show the task creation area
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
                        ui.label("2. Set priority (any letter A-Z) if any is wanted. A is the highest and Z the lowest proirity.");
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
                            let usr_change_pos_entry: String = String::new();
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
                            self.usr_change_pos_in.insert(index, usr_change_pos_entry);
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
                            let temp = TaskEncoder::encode_taskwidget(self.clone());
                            let path: PathBuf = self.file_path.clone();
                            let _ = TaskEncoder::save(temp, path);
                        }
                    });
                    ui.end_row();
            }
            // Show the sorting area
            if self.show_main_sorting_area {
                ui.horizontal(|ui: &mut Ui| {
                    // This button does nothing; if clicked all text input looses focus
                    // so a search will happen.
                    let _dummy = ui.button("Search");
                    if self.show_no_results_found_text {
                        ui.heading("No results found!");
                    }
                });
                
                Grid::new(ui_main_area.id).show(ui, |ui: &mut Ui| {
                    // I don't understand how to set a custom style or spacing, so I
                    // guess this monstroity will have to do.
                    for number in 0..9 {
                        if number == 5 || number == 6 || number == 7 || number == 8 {
                            let out = Self::left_and_rightpad(25, "".to_string());
                            ui.label(out);
                        } else {
                            ui.label("");
                        }
                    }
                    ui.end_row();
                    ui.label("");
                    if ui.button("Reset search").clicked() {
                        self.usr_sort_completion = false;
                        self.usr_sort_create_date = false;
                        self.usr_sort_priority = false;
                        self.usr_sort_task_text_in = "Enter task text to search".to_string();
                        self.usr_sort_project_tags_in = "Enter +ProjectTags to search".to_string();
                        self.usr_sort_context_tags_in = "Enter @ContextTags to search".to_string();
                        self.usr_sort_special_tags_in = "Enter Special:Tags to search".to_string();
                        self.sort_tasks_indices = Vec::new();
                    }
                    // Radio buttons for true / false sorting.
                    // By completion: First by if completed, then by completion date if
                    // applicable.
                    if ui.radio(self.usr_sort_completion, "By completion").clicked() {
                        if !self.usr_sort_completion {
                            self.usr_sort_completion = true;
                        } else {
                            self.usr_sort_completion = false;
                        }
                        
                    }
                    if ui.radio(self.usr_sort_create_date, "By inception date").clicked() {
                        if !self.usr_sort_create_date {
                            self.usr_sort_create_date = true;
                        } else {
                            self.usr_sort_create_date = false;
                        }
                        
                    }
                    if ui.radio(self.usr_sort_priority, "By priority").clicked() {
                        if !self.usr_sort_priority {
                            self.usr_sort_priority = true;
                        } else {
                            self.usr_sort_priority = false;
                        }
                        
                    }
                    // Sorting logic for true / false sorting
                    if self.usr_sort_completion || self.usr_sort_create_date || self.usr_sort_priority {
                        self.sort_true_false();
                    }
                    // Text input for field searching
                    // Task text search
                    let task_text_in = ui.text_edit_multiline(&mut self.usr_sort_task_text_in);
                    if task_text_in.gained_focus() {
                        if self.usr_sort_task_text_in.contains("Enter task text to search") {
                            self.usr_sort_task_text_in = String::new();
                        }
                    } else if task_text_in.lost_focus() {
                        self.sort_task_text();
                        if self.sort_tasks_indices.len() < 1 {
                            self.show_no_results_found_text = true;
                        }
                    }
                    // Project tag search
                    let project_in = ui.text_edit_multiline(&mut self.usr_sort_project_tags_in);
                    if project_in.gained_focus() {
                        if self.usr_sort_project_tags_in.contains("Enter +ProjectTags to search") {
                            self.usr_sort_project_tags_in = String::new();
                        }
                    } else if project_in.lost_focus() {
                        println!("Lost focus!")
                    }
                    // context tag search
                    let context_in = ui.text_edit_multiline(&mut self.usr_sort_context_tags_in);
                    if context_in.gained_focus() {
                        if self.usr_sort_context_tags_in.contains("Enter @ContextTags to search") {
                            self.usr_sort_context_tags_in = String::new();
                        }
                    } else if context_in.lost_focus() {
                        println!("Lost focus!")
                    }
                    // speacial tag search
                    let special_in = ui.text_edit_multiline(&mut self.usr_sort_special_tags_in);
                    if special_in.gained_focus() {
                        if self.usr_sort_special_tags_in.contains("Enter Special:Tags to search") {
                            self.usr_sort_special_tags_in = String::new();
                        }
                    } else if special_in.lost_focus() {
                        println!("Lost focus!")
                    }
                    ui.label("");
                    ui.end_row();
                    for _ in [0..8] {
                        ui.label("");
                    }
                });
                // WIP: most used tags
                let mut temp = false;
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Most used context tags:");
                    // I'm are going to use buttons! -> they don't need Appstate
                    // allocation
                    ui.button("the first tag");
                    ui.checkbox(&mut temp, "second tag");
                });
                ui.label("Most used context tags:");
                ui.label("Most used special tags:");
            }
            // Shows the about text
            if self.show_main_panel_about_text {
                ui.heading("About Ananke");
                ui.label("Ananke is a fully-featured, end-to-end, zero-to-one Todo app that leverages the power of the todo.txt format to provide a seamless, frictionless and streamlined user experience. Built on a solid foundation of cutting-edge technologies, rust.");
                ui.label("Ananke decodes your todo.txt, makes it look pretty and searchable, as well as creates new tasks, and updates finished ones.");
                ui.heading("About the format todo.txt");
                ui.label("The todo.txt format is a plain text format file for managing tasks. It is at it's core really only a .txt file named todo. It contains one task per line, and each task line can contain infomation like: A priority letter (A-Z) first, then the Inception (Creation) and Completion dates in (YYYY-MM-DD format), Project Tags (preceeded by the + sign), Context Tags (preceeded by the @ sign), and finally Special tags that only follow the [keyTag:AnyContentYouWantToBeSearchableWithTheKeyTag].");
                ui.heading("Licenses");
                ui.hyperlink_to(format!("egui licensed under the MIT-License"), "https://github.com/emilk/egui/blob/master/LICENSE-MIT");
                    }
            // display the main task scrollable area.
            if self.show_task_scroll_area {
                ScrollArea::vertical().show(ui, |ui| {
                    // I want the main welcome text to be inside the scrollable area. In contrast
                    // to everything else in the top slot.
                    if self.show_main_panel_welcome_text {
                        ui.heading(format!("Ananke - todo.txt editor"));
                        ui.label(format!("by {AUTHOR}, v. {VERSION}"));
                        ui.hyperlink_to(format!("{NAME} on github"), "https://github.com/Xqhare/ananke");
                    }
                    // This is all for displaying the grid of tasks.
                    let vec_strings = vec!["#".to_string(), "Completed".to_string(), "Completion date".to_string(), "Inception date ".to_string(), "Priority".to_string(), "Task".to_string(), "Project  Tags".to_string(), "Context  Tags".to_string(), "Special  Tags".to_string()];
                    let task_list_seperator = ui.separator();
                    let _a_grid = Grid::new(task_list_seperator.id).striped(true).show(ui, |ui| {
                        // Drawing the collum names
                        for mut name in vec_strings {
                            if name.contains("Completed") && self.show_task_deletion_collum {
                                name = "Delete".to_string();
                            }
                            if name.contains("Completed") && self.show_task_move_pos_collum {
                                name = "Move task to #".to_string();
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
                        let mut display_task_indicies: Vec<usize> = Vec::new();
                        if self.sort_tasks_indices.len() > 0 {
                            display_task_indicies = self.sort_tasks_indices.clone();
                        } else {
                            for num in 0..self.tasks_vec.len() {
                                display_task_indicies.push(num);
                            }
                        }
                        for entry in display_task_indicies {
                            ui.label(entry.to_string());
                            let text = "Done!";
                            // The to be changed struct member HAS TO BE INSIDE the ui call! Got it!
                            // If task is marked as completed AND has a a creation date set, we set
                            // a completion date.
                            if self.show_task_deletion_collum {
                                if ui.button("Delete").clicked() {
                                    // Delete entry at counter!
                                    delete_entry = true;
                                    delete_pos.push(entry);
                                }
                            } else if self.show_task_move_pos_collum {
                                if ui.text_edit_singleline(&mut self.usr_change_pos_in[entry]).lost_focus() {
                                    let mut vec_out: Vec<(usize, usize)> = Vec::new();
                                    // This could technichally panic because of unwrap;
                                    // however it is only called if the value to be
                                    // unwraped is `Ok()`. 
                                    if self.usr_change_pos_in[entry].parse::<usize>().is_ok() {
                                        let new_pos = self.usr_change_pos_in[entry].parse::<usize>().unwrap();
                                        vec_out.push((entry.clone(), new_pos))
                                    }
                                    self.change_task_touple = (true, vec_out);
                                    // resetting user input field after decoding and saving of contents
                                    self.usr_change_pos_in[entry] = String::new();
                                }
                            } else {
                                if ui.checkbox(&mut self.completed_vec[entry], text).clicked() {
                                    if self.completed_vec[entry] {
                                        if !self.create_date_vec[entry].is_empty() {
                                            let date_today = self.date.clone();
                                            self.complete_date_vec.remove(entry);
                                            self.complete_date_vec.insert(entry, date_today);
                                        }
                                    } else {
                                        self.complete_date_vec.remove(entry);
                                        self.complete_date_vec.insert(entry, String::new());
                                    }
                                }
                            }
                            
                            // completion and creation dates
                            ui.text_edit_singleline(&mut self.complete_date_vec[entry]);
                            ui.text_edit_singleline(&mut self.create_date_vec[entry]);
                            // Priority implementation
                            // variable input fields are very versitile!
                            ui.text_edit_singleline(&mut self.priority_vec[entry]);
                            ui.text_edit_multiline(&mut self.task_text[entry]);
                            // Da tags!!
                            ui.text_edit_multiline(&mut self.project_tags_vec[entry]);
                            ui.text_edit_multiline(&mut self.context_tags_vec[entry]);
                            ui.text_edit_multiline(&mut self.special_tags_vec[entry]);
                            // End of task; -> end the row
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
        if self.delete_task_touple.0 {
            let mut counter: usize = 0;
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
                counter += 1;
            }
            // resetting the delete task buffer
            if self.delete_task_touple.1.len() == counter {
                let empty_touple_out: (bool, Vec<usize>) = (false, Vec::new());
                self.delete_task_touple = empty_touple_out;
            }
        }
        if self.change_task_touple.0 {
            let mut counter: usize = 0;
            for element in &mut self.change_task_touple.1 {
                let old_pos = element.0.clone();
                let new_pos = element.1.clone();
                // remove -> insert loop
                let task_widget = self.tasks_vec.remove(old_pos);
                self.tasks_vec.insert(new_pos, task_widget);
                let completion = self.completed_vec.remove(old_pos);
                self.completed_vec.insert(new_pos, completion);
                let prio = self.priority_vec.remove(new_pos);
                self.priority_vec.insert(new_pos, prio);
                let comp_date = self.complete_date_vec.remove(old_pos);
                self.complete_date_vec.insert(new_pos, comp_date);
                let create_date = self.create_date_vec.remove(old_pos);
                self.create_date_vec.insert(new_pos, create_date);
                let task = self.task_text.remove(old_pos);
                self.task_text.insert(new_pos, task);
                let project_tag = self.project_tags_vec.remove(old_pos);
                self.project_tags_vec.insert(new_pos, project_tag);
                let context_tag = self.context_tags_vec.remove(old_pos);
                self.context_tags_vec.insert(new_pos, context_tag);
                let special_tag = self.special_tags_vec.remove(old_pos);
                self.special_tags_vec.insert(new_pos, special_tag);
                counter += 1;
            }
            // resetting the move buffer
            if self.change_task_touple.1.len() == counter {
                let empty_task_touple: (bool, Vec<(usize, usize)>) = (false, Vec::new());
                self.change_task_touple = empty_task_touple;
            }
        }
        self.main_panel(ctx, frame);
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

