use std::{io::{Error, Write}, fs, path::PathBuf};

use unicode_segmentation::UnicodeSegmentation;

use crate::gui::TaskWidget;

// To deconstruct a todo.txt task:
// Each task is on one line
// whitespace splits the elements
// if line starts with x+whitespace == completed
    // put at bottom/do not show
// Priority is in the format: (A-Z)
    // It should be discarded after task completion - for better automatic sorting of the tasks by completion, then date; Some clients transform it into a special tag e.g.
        // pri:A
// Dates in format YYYY-MM-DD
    // If completion date is specified, creation time has to be specified too.
    // for simplicity I could just always add the creation date; - as a special tag!
// Normal text has no special char at the beginning, but can have any char inside it.
    // e.g. normal text means one can also use numb3rs 456 and things: like-this
    // IMPLEMENTAION OPTIONAL
        // calculations are possible with the = prefix e.g.
        // =50*32 or more complex.
// Project tags start with a +
// Context tags with @
// and special tags follow -> key:value
    // here don't forget to check if it's 'word: more text' vs 'word:text'
    // first would be text, second a special tag
// interesting special tags to add:
// - due:YYYY-MM-DD
// - pri:A
// - created:YYYY-MM-DD

/// The struct that decodes and sorts the todo.txt input.
/// The fields are in format order.
/// This struct doesn't panik if supplied with an any-length String.
#[derive(Debug, Clone)]
pub struct TaskDecoder {
    /// Task completion
    pub completed: Option<bool>,
    /// Task priority if set
    pub priority: Option<String>,
    /// Task completion date if set
    pub complete_date: Option<String>,
    /// Task creation date if set - Has to be set if completion date is to be set.
    pub create_date: Option<String>,
    /// Main task text
    pub task: String,
    /// Task project tags as a vector, if any present
    pub project_tags: Option<Vec<String>>,
    /// Task context tags as a vector, if any present
    pub context_tags: Option<Vec<String>>,
    /// Task special tags as a vector, if any present
    pub special_tags: Option<Vec<String>>,
}

/// In the Implementation, there is only the `new()` function for decoding
impl TaskDecoder {
    /// This function decodes a line of todo.txt formatted text and returns the Task struct for
    /// interrigation and doesn't panik if supplied with a 0 length string.
    pub fn new(task_to_decode: String) -> Self {

        let mut completed = Option::from(false);
        let mut priority: Option<String> = Option::default();
        let mut complete_date: Option<String> = Option::default();
        let mut create_date: Option<String> = Option::default();
        let mut task = String::new();
        let mut project_tags: Option<Vec<String>> = Option::default();
        let mut context_tags: Option<Vec<String>> = Option::default();
        let mut special_tags: Option<Vec<String>> = Option::default();

        let mut date_number = 0;

        for item in task_to_decode.split_whitespace() {
            if item.starts_with("x") {
                completed = Option::Some(true)
            } else if item.starts_with("(") && item.ends_with(")") && item[..].graphemes(true).count() == 3 {
                let output = item.replace("(", "").replace(")", "").to_uppercase();
                priority = Option::Some(output)
            } else if  item.starts_with("(") && item.ends_with(")") && item[..].graphemes(true).count() == 2 {
                priority = Option::None
            // If three blocks of anything are split by `-` I'll just assume its a date.
            } else if item.split("-").count() == 3 && !item.contains(":") {
                // First date incountered
                if date_number == 0 {
                    date_number += 1;
                    complete_date = Option::Some(item.to_string())
                } else if date_number == 1 {
                    date_number += 1;
                    create_date = Option::Some(item.to_string())
                }
            // Project tags
            } else if item.starts_with("+") {
                let mut output = project_tags.unwrap_or(Vec::new());
                output.push(item.to_string());
                project_tags = Option::Some(output)
            // Context tags
            } else if item.starts_with("@") {
                let mut output = context_tags.unwrap_or(Vec::new());
                output.push(item.to_string());
                context_tags = Option::Some(output)
            // Special tags
            } else if item.contains(":") && !item.ends_with(":") {
                let mut output = special_tags.unwrap_or(Vec::new());
                output.push(item.to_string());
                special_tags = Option::Some(output)
            // Maths 
                // Now TECHNICALLY the .txt file should not be changed and = x +- y should stay...
                // There is however really no reason not to just compute it and put the final in.
            } else if item.starts_with("=") {
                let maths_output = mexprp::eval::<f64>(&item.replace("=", "")).unwrap();
                task.push_str(&maths_output.to_string());
                task.push(' ');
            // Normal text
            } else {
                task.push_str(item);
                task.push(' ');
            }
        }
        // If task is not completed, date_number changes; meaning that complete becomes create_date;
        if date_number == 1 {
            create_date = complete_date;
            let reset_var: Option<String> = Option::default();
            complete_date = reset_var;
        }

        return TaskDecoder{completed, priority, complete_date, create_date, task, project_tags, context_tags, special_tags};
    }
}

#[derive(Clone)]
struct Task {
    row: String,
}

impl Task {
    fn new(input: String) -> Self {
        Self { row: input }
    }
}

#[derive(Clone)]
pub struct TaskEncoder {
    rows: Vec<Task>,
}

/// This implements the encoding to real todo.txt formatted output for the save-file, from the Appstate within `TaskWidget`.
impl TaskEncoder {
    /// Can be thought of as the `default()` for `TaskEncoder`.
    pub fn encode_taskwidget(widget: TaskWidget) -> Self {
        let mut completed_tasks: Vec<Task> = Vec::new();
        let mut output: Vec<Task> = Vec::new();
        let mut counter: usize = 0;
        for _entry in widget.tasks_vec.clone() {
            // format demands completed tasks to be put last
            if widget.completed_vec[counter] == false {
                let encoded_single_task: String = Self::encode_single_taks(widget.clone(), counter);
                let task_out = Task::new(encoded_single_task);
                output.push(task_out);
            } else {
                let encoded_single_task: String = Self::encode_single_taks(widget.clone(), counter);
                let task_out = Task::new(encoded_single_task);
                completed_tasks.push(task_out);
            }
            // advancing counter as last step
            counter += 1;
        }
        if completed_tasks.len() > 0 {
            for entry in completed_tasks{
                output.push(entry);
            }
        }
        // Debug / Sanity printout
        for entry in &output {
            println!("{:?}", entry.row);
        }
        Self { rows: output }
    }
    /// Takes in the `TaskWidget` (Appstate) and a `position`, and returns the finished encoded string of the task
    /// at `position`.
    fn encode_single_taks(input_task: TaskWidget, position: usize) -> String {
        let mut output: String = String::new();
        let str_spacer: &str = " ";
        // completion first - Spacing built in
        let completion = match input_task.completed_vec[position.clone()] {
            true => String::from("x "),
            _ => String::new(),
        };
        output.push_str(&completion);
        // priority
        let temp_prio = input_task.priority_vec[position.clone()].clone();
        let priority = if temp_prio.graphemes(true).count() <= 1 {
                    let prio = temp_prio.to_ascii_uppercase();
                    let mut prio_out = format!("({prio})");
                    prio_out.push_str(str_spacer);
                    prio_out
                } else if input_task.priority_vec[position].graphemes(true).count() > 1 {
                    let prio = temp_prio.to_ascii_uppercase();
                    let shortend_prio = prio.graphemes(true).take(1).last().unwrap();
                    let mut prio_out = format!("({shortend_prio})");
                    prio_out.push_str(str_spacer);
                    prio_out
                // This really is only a failsafe - And it makes the LSP shut up.
                } else {
                    String::new()
        };
        output.push_str(&priority);
        // completion and creation date
        let completion_date = if input_task.complete_date_vec[position].graphemes(true).count() > 0 {
            let mut out = input_task.complete_date_vec[position].clone();
            out.push_str(str_spacer);
            out
        } else {
            let out = String::new();
            out
        };
        output.push_str(&completion_date);
        let creation_date = if input_task.create_date_vec[position].graphemes(true).count() > 0 {
            let mut out = input_task.create_date_vec[position].clone();
            out.push_str(str_spacer);
            out
        } else {
            let out = String::new();
            out
        };
        output.push_str(&creation_date);
        // Main text of task
        let mut task_text = input_task.task_text[position].clone();
        task_text.push_str(str_spacer);
        output.push_str(&task_text);
        // Da tags! Project - Context - Special
        let mut project_tag = input_task.project_tags_vec[position].clone();
        project_tag.push_str(str_spacer);
        output.push_str(&project_tag);
        let mut context_tag = input_task.context_tags_vec[position].clone();
        context_tag.push_str(str_spacer);
        output.push_str(&context_tag);
        let mut special_tag = input_task.special_tags_vec[position].clone();
        special_tag.push_str(str_spacer);
        output.push_str(&special_tag);
        
        // Workaround to remove double spaces and trailing whitespace
        let final_out = output.replace("  ", " ").trim_end().to_string();
        return final_out;
    }
    pub fn save(self, filename: PathBuf) -> Result<(), Error> {
        let mut file = fs::File::create(filename)?;
        for row in self.rows {
            file.write_all(row.row.as_bytes())?;
            // for the newline
            file.write_all(b"\n")?;
        }
        Ok(())
    }
}
