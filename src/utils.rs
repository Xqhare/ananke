use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use talos::widgets::stateful::{ButtonState, States};

use crate::startup::Environment;

pub fn fps_sleeper(last_frame: Instant) -> (Instant, u128) {
    let fps_goal = 2_000;
    let ms_goal = 1_000_000 / fps_goal;
    let now = Instant::now();
    let last_frame_dur = now.duration_since(last_frame).as_micros();
    if last_frame_dur < ms_goal {
        sleep(Duration::from_micros((ms_goal - last_frame_dur) as u64));
    }
    (now, last_frame_dur)
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
            if k.contains("header_file_menu_sub_forget_button_") && !k.contains("state") {
                i += 1;
            }
            #[cfg(debug_assertions)]
            if k.contains("header_file_menu_sub_load_button") && !k.contains("state") {
                j += 1;
            }
        }
        debug_assert!(i == j);
        i
    };

    // Create the forget & load button + Update the path amount
    env.path_amount = i;
    env.states.insert(
        format!("header_file_menu_sub_forget_button_{i}"),
        States::from(ButtonState { clicked: false }),
    );
    env.states.insert(
        format!("header_file_menu_sub_load_button_{i}"),
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
