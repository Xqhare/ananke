use unicode_segmentation::UnicodeSegmentation;

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
pub struct Task {
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
impl Task {
    /// This function decodes a line of todo.txt formatted text and returns the Task struct for
    /// interrigation
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

        return Task{completed, priority, complete_date, create_date, task, project_tags, context_tags, special_tags};
    }
}
