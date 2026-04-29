use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use talos::widgets::stateful::{ButtonState, States};

#[cfg(debug_assertions)]
use crate::keys::HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE;
use crate::{
    input::{CreatorFocus, Focus, MenuFocus},
    keys::{
        CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE, CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
        CREATOR_TASK_ENTRY_TEXTBOX_STATE, HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE,
        HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE, MENU_SEARCH_PRIO_TEXTBOX,
        MENU_SEARCH_PRIO_TEXTBOX_STATE, MENU_SEARCH_TEXTBOX, MENU_SEARCH_TEXTBOX_STATE,
    },
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
    // Create the forget & load button
    let i = {
        // Find the index of the last forget button

        let map = env.states.iter().map(|(k, _)| k).collect::<Vec<_>>();

        let mut i = 0;

        // Just to be sure - But for performance we only
        // check both i and j when debugging
        #[cfg(debug_assertions)]
        let mut j = 0;

        for k in map {
            if k.contains(HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE) {
                i += 1;
            }
            #[cfg(debug_assertions)]
            if k.contains(HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE) {
                j += 1;
            }
        }
        // If `debug_assert` is used, the compiler still complains about the missing inner attribute
        #[cfg(debug_assertions)]
        assert!(i == j);

        // `i` is the current amount of forget and load buttons,
        // To add one more button, increment `i` by one
        i + 1
    };

    // Create the forget & load button + Update the path amount
    env.path_amount = i;
    env.states.insert(
        format!("{HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE}{i}"),
        States::from(ButtonState { clicked: false }),
    );
    env.states.insert(
        format!("{HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE}{i}"),
        States::from(ButtonState { clicked: false }),
    );
}

/// Toggle a button
///
/// # Arguments
///
/// * `env` - The environment
/// * `name` - The name of the button
///
/// # Returns
///
/// The state of the button - clicked or not
pub fn toggle_button(env: &mut Environment, name: &str) -> bool {
    let state = env.states.get_mut(name).unwrap().as_button_mut().unwrap();
    state.clicked = !state.clicked;
    state.clicked
}

pub fn toggle_dropdown(env: &mut Environment, name: &str) {
    let state = env.states.get_mut(name).unwrap().as_dropdown_mut().unwrap();
    state.expanded = !state.expanded;
}

/// Ensures that the focus is on the active textfield only
///
/// Does so by hooking into the `Focus` enum used for keyboard input capture
pub fn ensure_focus_on_active_textfield(env: &mut Environment, focus: &Focus) {
    match focus {
        Focus::None => {
            // Clear the focus of any textfield
            let states = vec![
                HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE,
                CREATOR_TASK_ENTRY_TEXTBOX_STATE,
                CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
                CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE,
                MENU_SEARCH_TEXTBOX_STATE,
                MENU_SEARCH_PRIO_TEXTBOX_STATE,
            ];
            for state in states {
                env.states
                    .get_mut(state)
                    .unwrap()
                    .as_text_box_mut()
                    .unwrap()
                    .active = false;
            }
        }
        Focus::HeaderFileNewTextBox => {
            // Ensure that the focus is on the textfield
            env.states
                .get_mut(HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap()
                .active = true;
        }
        Focus::Creator(CreatorFocus::Task) => {
            env.states
                .get_mut(CREATOR_TASK_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap()
                .active = true;
        }
        Focus::Creator(CreatorFocus::Priority) => {
            env.states
                .get_mut(CREATOR_PRIO_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap()
                .active = true;
        }
        Focus::Creator(CreatorFocus::CreationDate) => {
            env.states
                .get_mut(CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap()
                .active = true;
        }
        Focus::Menu(MenuFocus::Text) => {
            env.states
                .get_mut(MENU_SEARCH_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap()
                .active = true;
        }
        Focus::Menu(MenuFocus::Priority) => {
            env.states
                .get_mut(MENU_SEARCH_PRIO_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap()
                .active = true;
        }
    }
}
