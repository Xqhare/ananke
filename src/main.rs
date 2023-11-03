#![feature(const_trait_impl)]
//! `ananke`: a todo.txt editor in pure Rust!
//! The name Ananke is derived from the Greek goddess of necessity and inevitability.
//!
//! Source code can be found on [`github`].
//!
//! [`github`]: https://github.com/Xqhare/ananke

use std::{path::PathBuf, env, fs::{File, self}, ffi::OsString, io::Write, os::unix::prelude::OsStrExt};

/// Contains the Appstate, rendering, styling and saving.
mod gui;
/// Used to decode or encode a line of todo.txt formatted text.
mod task;

// Nice to have's:
// WIP: user's customised todo.txt location - persistant between restarts?

/// The main function only calles the `gui::main()` function.
fn main() {
    // println!("Welcome to {NAME} by {AUTHOR}, v. {VERSION}");
    gui::main();
}

// Problem: persistant file location,
// Solution: Usr drops in a file -> PathBuf, PathBuf is opened for ananke and saved as file in .folder in home
// 1. Function to determine IF a file has been written before it, if so opening that, if not
//    continue:
// 2. Function that takes in the PathBuf, hands it off to a save function AND openes/reloads ananke (if possible)
/// This function checks if an appstate exists and returns a touple containing true if it does,
/// false if not. It also returns the appstate directory, the appstate file name, and both put
/// together, in that order, as `PathBuf`.
pub fn check_for_persistant_appstate() -> (bool, PathBuf) {
    let mut appstate_dir_os_string: OsString = OsString::new();
    let appstate_file_name: OsString = OsString::from(".ananke_config");
    appstate_dir_os_string.push(appstate_file_name);
    let appstate_dir_pathbuf = PathBuf::from(appstate_dir_os_string);
    if appstate_dir_pathbuf.try_exists().is_ok() {
        // appstate exists
        (true, appstate_dir_pathbuf)
    } else {
        // appstate doesn't exist
        (false, appstate_dir_pathbuf)
    }
}
pub fn create_persistant_appstate(full_appstate_path_name: PathBuf, todo_file_path: PathBuf) {
    let mut new_appstate = File::create(full_appstate_path_name).expect("Something went terribly wrong, main.rs create_persistant_appstate");
    let out = todo_file_path.into_os_string();
    let _ignore_errot = new_appstate.write_all(out.as_bytes());
}

