use anansi::List;
use brigid::content::Content;
use nabu::{Object, XffValue, xff};
use talos::{
    codex::Codex,
    input::{KeyCode, KeyEvent},
};

use crate::{
    input::{
        Focus,
        creator::{update_new_task_after_key_press, update_render_list},
    },
    keys::{
        CREATOR_PRIO_ENTRY_TEXTBOX, CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
        CREATOR_TASK_ENTRY_TEXTBOX_STATE, HEADER_EXIT_BUTTON, HEADER_FILE_MENU_BUTTON,
        HEADER_FILE_MENU_BUTTON_STATE, HEADER_FILE_MENU_SUB_FORGET_BUTTON,
        HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE, HEADER_FILE_MENU_SUB_FORGET_BUTTON_STATE,
        HEADER_FILE_MENU_SUB_LOAD_BUTTON, HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE,
        HEADER_FILE_MENU_SUB_LOAD_BUTTON_STATE, HEADER_FILE_MENU_SUB_NEW_BUTTON,
        HEADER_FILE_MENU_SUB_NEW_BUTTON_STATE, HEADER_FILE_MENU_SUB_NEW_TEXTBOX,
        HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE, HEADER_HELP_BUTTON, HEADER_HELP_BUTTON_STATE,
        HEADER_SAVE_BUTTON, MENU_SEARCH_PRIO_TEXTBOX, MENU_SEARCH_PRIO_TEXTBOX_STATE,
        MENU_SEARCH_TEXTBOX, MENU_SEARCH_TEXTBOX_STATE,
    },
    startup::Environment,
    utils::{goto_exit, toggle_button},
};

/// Handles the key events for the new file text box
pub fn handle_key_textbox_newfile(
    name: &str,
    key_event: &KeyEvent,
    env: &mut Environment,
    codex: &Codex,
) -> Option<()> {
    // First get the state once
    let state = env.states.get_mut(name).unwrap().as_text_box_mut().unwrap();
    let mut content = state.text.get_content().to_string();

    let mut update_creator_after_key_press = false;
    let mut update_r_list = false;
    // Now handle the key
    match key_event.code {
        KeyCode::Enter => {
            // As there is only one text box inside the header, we don't need to check
            // the name that thoroughly.
            if name.contains("header") {
                // Just to be sure, flush one last time & save
                state.text.set_content(&content, codex);
                let _ = env.list.save().unwrap();

                // Update the list
                env.list = List::new(&content);
                update_r_list = true;

                // Save the path
                let mut path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let obj = path.as_object_mut().unwrap();
                let paths_ary = obj.get_mut("paths").unwrap();
                paths_ary.as_array_mut().unwrap().push(xff!(&content));
                let _ = env
                    .disk_env
                    .brigid
                    .update_file("config.xff", Content::XFF(xff!(obj.clone())));

                // Reset the text box
                state.active = false;
                state
                    .text
                    .set_content(env.disk_env.home_path.to_string_lossy(), codex);
            } else if !update_r_list {
                return Some(());
            }
        }
        KeyCode::Backspace => {
            content.pop();
        }
        KeyCode::Char(c) => {
            content.push(c);
            if name == CREATOR_TASK_ENTRY_TEXTBOX_STATE {
                env.new_task.update_text(&content);
                update_creator_after_key_press = true;
            }
        }
        _ => {}
    }

    // Update the text with the new content
    state.text.set_content(&content, codex);
    state.cursor = Some(state.text.len());

    if name == CREATOR_PRIO_ENTRY_TEXTBOX_STATE {
        if keep_textbox_at_one_char(env, name, codex) {
            let state = env
                .states
                .get_mut(CREATOR_PRIO_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = false;
        }
    } else if name == MENU_SEARCH_PRIO_TEXTBOX_STATE {
        if keep_textbox_at_one_char(env, name, codex) {
            let state = env
                .states
                .get_mut(MENU_SEARCH_PRIO_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = false;
        }
    }

    // Update state after each key entry where it makes sense to make UI feel snappy and
    // responsive
    if name == MENU_SEARCH_TEXTBOX_STATE || name == MENU_SEARCH_PRIO_TEXTBOX_STATE {
        update_render_list(env);
    }
    if update_creator_after_key_press {
        update_new_task_after_key_press(env, codex);
    }
    if update_r_list {
        update_render_list(env);
        return Some(());
    }
    None
}

/// # Returns
/// If the textbox was shrunk to one character or not. True: was shrunk, False: was not
fn keep_textbox_at_one_char(env: &mut Environment, name: &str, codex: &Codex) -> bool {
    let state = env.states.get_mut(name).unwrap().as_text_box_mut().unwrap();
    if state.text.len() > 1 {
        state.text.set_content(
            state
                .text
                .get_content()
                .chars()
                .take(1)
                .next()
                .unwrap()
                .to_uppercase()
                .take(1)
                .next()
                .unwrap(),
            codex,
        );
        true
    } else if state.text.len() == 1 {
        state
            .text
            .set_content(state.text.get_content().to_uppercase(), codex);
        false
    } else {
        false
    }
}

/// Handles the mouse events for the header
pub fn handle_header_mouse(env: &mut Environment, name: &str) -> Focus {
    match name {
        HEADER_FILE_MENU_BUTTON => {
            toggle_button(env, HEADER_FILE_MENU_BUTTON_STATE);
            Focus::None
        }
        HEADER_FILE_MENU_SUB_NEW_BUTTON => {
            if toggle_button(env, HEADER_FILE_MENU_SUB_NEW_BUTTON_STATE) {
                let state = env
                    .states
                    .get_mut(HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE)
                    .unwrap()
                    .as_text_box_mut()
                    .unwrap();
                state.active = true;
                Focus::HeaderFileNewTextBox
            } else {
                Focus::None
            }
        }
        HEADER_FILE_MENU_SUB_LOAD_BUTTON => {
            toggle_button(env, HEADER_FILE_MENU_SUB_LOAD_BUTTON_STATE);
            Focus::None
        }
        HEADER_FILE_MENU_SUB_FORGET_BUTTON => {
            toggle_button(env, HEADER_FILE_MENU_SUB_FORGET_BUTTON_STATE);
            Focus::None
        }
        HEADER_FILE_MENU_SUB_NEW_TEXTBOX => {
            let state = env
                .states
                .get_mut(HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = !state.active;
            // If a user clicks into the textbox directly, we want to always focus it
            Focus::HeaderFileNewTextBox
        }
        HEADER_SAVE_BUTTON => {
            // Save the list - Nothing else to do; More of a pseudo button
            env.list.save().unwrap();
            Focus::None
        }
        HEADER_HELP_BUTTON => {
            toggle_button(env, HEADER_HELP_BUTTON_STATE);
            Focus::None
        }
        HEADER_EXIT_BUTTON => {
            // No need to set the button state; just exit this frame
            goto_exit(env);
            Focus::None
        }
        _ => {
            // Handle the buttons with an arbitrary amount
            //
            // Also load only if the name contains the base, to minimise IO calls
            if name.contains(HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE) {
                let mut path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let obj = path.as_object_mut().unwrap();
                handle_mouse_forget_button(name, env, obj);
            } else if name.contains(HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE) {
                let mut path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let obj = path.as_object_mut().unwrap();
                handle_mouse_load_button(name, env, obj);
            }
            Focus::None
        }
    }
}

/// Handles the mouse events for the forget button
fn handle_mouse_forget_button(name: &str, env: &mut Environment, obj: &mut Object) {
    // Close the forget menu
    let button = env
        .states
        .get_mut(HEADER_FILE_MENU_SUB_FORGET_BUTTON_STATE)
        .unwrap()
        .as_button_mut()
        .unwrap();
    button.clicked = false;

    let index = name.split("_").last().unwrap().parse::<u32>().unwrap();
    let paths_ary = obj.get_mut("paths").unwrap();
    let path = paths_ary.as_array().unwrap().get(index as usize).unwrap();
    let path = path.as_string().unwrap();
    // Don't forget the default list - never delete that!
    if !path.contains("/Ananke/default-list.txt") {
        // Remove the button states
        env.states
            .remove_entry(&format!("{HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE}{index}"));
        env.states
            .remove_entry(&format!("{HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE}{index}"));

        // Remove the path
        let paths_ary = paths_ary.as_array_mut().unwrap();
        paths_ary.remove(index as usize);
        debug_assert!(paths_ary.len() > 0);

        // Update the environment & config
        env.list = List::load(paths_ary.get(0).unwrap().as_string().unwrap()).unwrap();
        env.path_amount -= 1;
        let _ = env
            .disk_env
            .brigid
            .update_file("config.xff", Content::XFF(xff!(obj.clone())));
    }
}

/// Handles the mouse events for the load button
fn handle_mouse_load_button(name: &str, env: &mut Environment, obj: &mut Object) {
    // Close the load menu
    let button = env
        .states
        .get_mut(HEADER_FILE_MENU_SUB_LOAD_BUTTON_STATE)
        .unwrap()
        .as_button_mut()
        .unwrap();
    button.clicked = false;

    // Save the current list
    let _ = env.list.save().unwrap();

    // Construct the path
    let index = name.split("_").last().unwrap().parse::<u32>().unwrap();
    let path = obj.get("paths").unwrap();
    let path = path.as_array().unwrap().get(index as usize).unwrap();
    let path = path.as_string().unwrap();

    // Load the new list if it's different and update the config
    if env.list.get_path() != path {
        env.list = List::load(path).unwrap();
    }
    let _ = env
        .disk_env
        .brigid
        .update_file("config.xff", Content::XFF(xff!(obj.clone())));
}
