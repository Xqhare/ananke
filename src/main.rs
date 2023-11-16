#![feature(const_trait_impl)]
//! `ananke`: a todo.txt editor in pure Rust!
//! The name Ananke is derived from the Greek goddess of necessity and inevitability.
//!
//! Source code can be found on [`github`].
//!
//! [`github`]: https://github.com/Xqhare/ananke

use std::{path::PathBuf, fs::File, ffi::OsString, io::Write, os::unix::prelude::OsStrExt, collections::HashMap};

/// Contains the Appstate, rendering, styling and saving.
mod gui;
/// Used to decode or encode a line of todo.txt formatted text.
mod task;

/// The main function only calles the `gui::main()` function.
fn main() {
    // println!("Welcome to {NAME} by {AUTHOR}, v. {VERSION}");
    gui::main();
}
// This short description of the todo.txt format needed someplace for quick reference, but task.rs
// really wasn't the right place:
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

/// This function checks if an appstate exists and returns a touple containing true if it does,
/// false if not. It also returns the appstate directory, the appstate file name, and both put
/// together, in that order, as `PathBuf`.
pub fn check_for_persistant_appstate() -> (bool, PathBuf) {
    let mut appstate_dir_os_string: OsString = OsString::new();
    let appstate_file_name: OsString = OsString::from(".anankeconfig");
    appstate_dir_os_string.push(appstate_file_name);
    let appstate_dir_pathbuf = PathBuf::from(appstate_dir_os_string);
    match appstate_dir_pathbuf.try_exists() {
        Ok(result) => {
            if result {
                // appstate exists
                return (true, appstate_dir_pathbuf);
            } else {
                // no appstate
                return (false, appstate_dir_pathbuf);
            }
        },
        _ => return (false, appstate_dir_pathbuf),
    };
}
/// Creates a file, and writes the data needed for the persistant appstate, in this case only the
/// path to the todo.txt file of the user. Takes in the path (filename) and thing to be written,
/// the todo file path.
pub fn create_persistant_appstate(full_appstate_path_name: PathBuf, todo_file_path: PathBuf) {
    let mut new_appstate = File::create(full_appstate_path_name).expect("Something went terribly wrong, main.rs create_persistant_appstate");
    let out = todo_file_path.into_os_string();
    let _ignore_errot = new_appstate.write_all(out.as_bytes());
}
/// Takes in a `String` and returns a Vector containing all words and the amount of times they were
/// used ordered by that amount.
///
/// ## Info
/// This is the first time I used an `HashMap`, so this explanation could be wrong:
/// 
/// ### Explanation
/// The input is split by whitespace, and then put as a key into the `HashMap`. The `or_insert` part counts
/// up if a key inside this map already exists and the count is put inside the `HashMap` aswell.
/// The `HashMap` is then collected into a `Vec` filled with the `touples` of the `String` with its
/// amount. 
/// That `Vec` is then sorted by the amount.
pub fn word_counts(input: String) -> Vec<(String, usize)> {
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    for word in input.split_whitespace() {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }
    let mut word_count_vec: Vec<(String, usize)> = word_counts.into_iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    return word_count_vec;
}
