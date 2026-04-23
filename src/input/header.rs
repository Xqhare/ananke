use anansi::List;
use brigid::content::Content;
use nabu::{Object, XffValue, xff};
use talos::{
    codex::Codex,
    input::{KeyCode, KeyEvent},
};

use crate::{input::Focus, startup::Environment, utils::toggle_button};

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
                state.text.set_content("path/to/file.txt", codex);
                return Some(());
            }
        }
        KeyCode::Backspace => {
            content.pop();
        }
        KeyCode::Char(c) => {
            content.push(c);
        }
        _ => {}
    }
    // Update the text with the new content
    state.text.set_content(content, codex);
    state.cursor = Some(state.text.len().saturating_sub(1));
    None
}

/// Handles the mouse events for the header
pub fn handle_header_mouse(env: &mut Environment, name: &str) -> Focus {
    match name {
        "header_file_menu_button" => {
            toggle_button(env, "header_file_menu_button_main_button_state");
            Focus::None
        }
        "header_file_menu_sub_new_button" => {
            if toggle_button(env, "header_file_menu_sub_new_button_state") {
                Focus::HeaderFileNewTextBox
            } else {
                Focus::None
            }
        }
        "header_file_menu_sub_load_button" => {
            toggle_button(env, "header_file_menu_sub_load_button_state");
            Focus::None
        }
        "header_file_menu_sub_forget_button" => {
            toggle_button(env, "header_file_menu_sub_forget_button_state");
            Focus::None
        }
        "header_file_menu_sub_new_textbox" => {
            let state = env
                .states
                .get_mut("header_file_menu_sub_new_textbox_state")
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = !state.active;
            // If a user clicks into the textbox directly, we want to always focus it
            Focus::HeaderFileNewTextBox
        }
        _ => {
            // Handle the buttons with an arbitrary amount
            if name.contains("header_file_menu_sub_forget_button_") {
                let mut path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let obj = path.as_object_mut().unwrap();
                handle_mouse_forget_button(name, env, obj);
            } else if name.contains("header_file_menu_sub_load_button_") {
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
    let index = name.split("_").last().unwrap().parse::<u32>().unwrap();
    let paths_ary = obj.get_mut("paths").unwrap();
    let path = paths_ary.as_array().unwrap().get(index as usize).unwrap();
    let path = path.as_string().unwrap();

    // Don't forget the default list - never delete that!
    if !path.contains("/Ananke/default-list.txt") {
        // Remove the button states
        env.states.remove_entry(name);
        env.states
            .remove_entry(&format!("header_file_menu_sub_load_button_{index}"));

        // Remove the path
        let paths_ary = paths_ary.as_array_mut().unwrap();
        paths_ary.remove(index as usize);
        debug_assert!(paths_ary.len() > 0);

        // Update the environment & config
        env.list = List::new(paths_ary.get(0).unwrap().as_string().unwrap());
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
        .get_mut("header_file_menu_sub_load_button_state")
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
        env.list = List::new(path);
    }
    let _ = env
        .disk_env
        .brigid
        .update_file("config.xff", Content::XFF(xff!(obj.clone())));
}
