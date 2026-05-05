use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use talos::widgets::stateful::ButtonState;

use crate::{
    input::{CreatorFocus, Focus, MenuFocus},
    startup::Environment,
};

pub fn fps_sleeper(last_frame: Instant) -> (Instant, u128) {
    let fps_goal = 2_000;
    let ms_goal = 1_000_000 / fps_goal;
    let last_frame_dur = Instant::now().duration_since(last_frame).as_micros();
    if last_frame_dur < ms_goal {
        sleep(Duration::from_micros((ms_goal - last_frame_dur) as u64));
    }
    let now = Instant::now();
    (now, now.duration_since(last_frame).as_micros())
}

/// Call the exit the program
///
/// Saves the state and sets run flag to false
pub fn goto_exit(env: &mut Environment) {
    // Set the flag
    env.run = false;
    // Save the list to be save and not loose data
    env.list.save().unwrap();
}

pub fn add_load_n_forget_button_states(env: &mut Environment) {
    // Create the forget & load button + Update the path amount
    env.path_amount += 1;
    env.ui_state
        .header
        .file_menu_dynamic_load_buttons
        .push(ButtonState { clicked: false });
    env.ui_state
        .header
        .file_menu_dynamic_forget_buttons
        .push(ButtonState { clicked: false });
}

/// Ensures that the focus is on the active textfield only
///
/// Does so by hooking into the `Focus` enum used for keyboard input capture
pub fn ensure_focus_on_active_textfield(env: &mut Environment, focus: &Focus) {
    // First, clear all focus
    env.ui_state.header.file_menu_sub_new_textbox.active = false;
    env.ui_state.creator.task_entry_textbox.active = false;
    env.ui_state.creator.prio_entry_textbox.active = false;
    env.ui_state.creator.creation_date_entry_textbox.active = false;
    env.ui_state.menu.search_textbox.active = false;
    env.ui_state.menu.sort_prio_textbox.active = false;

    match focus {
        Focus::None => {}
        Focus::HeaderFileNewTextBox => {
            env.ui_state.header.file_menu_sub_new_textbox.active = true;
        }
        Focus::Creator(CreatorFocus::Task) => {
            env.ui_state.creator.task_entry_textbox.active = true;
        }
        Focus::Creator(CreatorFocus::Priority) => {
            env.ui_state.creator.prio_entry_textbox.active = true;
        }
        Focus::Creator(CreatorFocus::CreationDate) => {
            env.ui_state.creator.creation_date_entry_textbox.active = true;
        }
        Focus::Menu(MenuFocus::Text) => {
            env.ui_state.menu.search_textbox.active = true;
        }
        Focus::Menu(MenuFocus::Priority) => {
            env.ui_state.menu.sort_prio_textbox.active = true;
        }
    }
}
