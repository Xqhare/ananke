//! `ananke`: a todo.txt editor in pure Rust!
//! The name Ananke is derived from the Greek goddess of necessity and inevitability.
//!
//! Source code can be found on [`github`].
//!
//! [`github`]: https://github.com/Xqhare/ananke

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

