#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gui::gui_startup;
use state::{persistent_state::PersistentState, startup_state::StartupState};

mod gui;
mod util;
mod state;
mod error;

const PERSISTENT_STATE_PATH: &str = "ananke_config.xff";

fn main() {
    if std::path::Path::new(PERSISTENT_STATE_PATH).exists() {
        let read = PersistentState::read_persistent(PERSISTENT_STATE_PATH);
        if let Err(e) = read {
            gui_startup(StartupState::new(PersistentState::default(), false, Some(e)));
        } else {
            gui_startup(StartupState::new(read.unwrap(), false, None));
        }
    } else {
        gui_startup(StartupState::new(PersistentState::default(), true, None));
    }
}
