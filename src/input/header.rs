use anansi::List;
use brigid::content::Content;
use nabu::{Object, XffValue, xff};
use talos::{
    codex::Codex,
    input::{KeyCode, KeyEvent},
};

use crate::{
    keys::{
        HEADER_EXIT_BUTTON, HEADER_FILE_MENU_BUTTON, HEADER_FILE_MENU_SUB_FORGET_BUTTON,
        HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE, HEADER_FILE_MENU_SUB_LOAD_BUTTON,
        HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE, HEADER_FILE_MENU_SUB_NEW_BUTTON,
        HEADER_FILE_MENU_SUB_NEW_TEXTBOX, HEADER_HELP_BUTTON, HEADER_SAVE_BUTTON,
    },
    startup::Environment,
    state::{CreatorFocus, Focus, MenuFocus},
    utils::goto_exit,
};

/// Handles the key events for the header new file logic
pub fn handle_header_newfile_input(
    key_event: &KeyEvent,
    env: &mut Environment,
    codex: &Codex,
) -> Option<()> {
    if key_event.code == KeyCode::Enter {
        let state = &mut env.ui_state.header.file_menu_sub_new_textbox;
        let content = state.text.get_content().to_string();

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
        state
            .text
            .set_content(env.disk_env.home_path.to_string_lossy(), codex);

        // Update the tasks to display
        env.render_tasks = env.list.tasks();

        // Reset the menu
        env.ui_state.reset_menu(codex);

        return Some(());
    }
    None
}

/// # Returns
/// If the textbox was shrunk to one character or not. True: was shrunk, False: was not
pub fn keep_textbox_at_one_char(env: &mut Environment, focus: &Focus, codex: &Codex) -> bool {
    let state = match focus {
        Focus::Creator(CreatorFocus::Priority) => &mut env.ui_state.creator.prio_entry_textbox,
        Focus::Menu(MenuFocus::Priority) => &mut env.ui_state.menu.sort_prio_textbox,
        _ => unreachable!(),
    };
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
pub fn handle_header_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    match name {
        HEADER_FILE_MENU_BUTTON => {
            env.ui_state.header.file_menu_button.clicked =
                !env.ui_state.header.file_menu_button.clicked;
            Focus::None
        }
        HEADER_FILE_MENU_SUB_NEW_BUTTON => {
            env.ui_state.header.file_menu_sub_new_button.clicked =
                !env.ui_state.header.file_menu_sub_new_button.clicked;
            if env.ui_state.header.file_menu_sub_new_button.clicked {
                env.ui_state.header.file_menu_sub_new_textbox.active = true;
                Focus::HeaderFileNewTextBox
            } else {
                Focus::None
            }
        }
        HEADER_FILE_MENU_SUB_LOAD_BUTTON => {
            env.ui_state.header.file_menu_sub_load_button.clicked =
                !env.ui_state.header.file_menu_sub_load_button.clicked;
            Focus::None
        }
        HEADER_FILE_MENU_SUB_FORGET_BUTTON => {
            env.ui_state.header.file_menu_sub_forget_button.clicked =
                !env.ui_state.header.file_menu_sub_forget_button.clicked;
            Focus::None
        }
        HEADER_FILE_MENU_SUB_NEW_TEXTBOX => {
            env.ui_state.header.file_menu_sub_new_textbox.active =
                !env.ui_state.header.file_menu_sub_new_textbox.active;
            // If a user clicks into the textbox directly, we want to always focus it
            Focus::HeaderFileNewTextBox
        }
        HEADER_SAVE_BUTTON => {
            // Save the list - Nothing else to do; More of a pseudo button
            env.list.save().unwrap();
            Focus::None
        }
        HEADER_HELP_BUTTON => {
            env.ui_state.header.help_button.clicked = !env.ui_state.header.help_button.clicked;
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
                handle_mouse_forget_button(name, env, obj, codex);
            } else if name.contains(HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE) {
                let mut path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let obj = path.as_object_mut().unwrap();
                handle_mouse_load_button(name, env, obj, codex);
            }
            Focus::None
        }
    }
}

/// Handles the mouse events for the forget button
fn handle_mouse_forget_button(name: &str, env: &mut Environment, obj: &mut Object, codex: &Codex) {
    // Close the forget menu
    env.ui_state.header.file_menu_sub_forget_button.clicked = false;

    let index = name.split("_").last().unwrap().parse::<usize>().unwrap();
    let paths_ary = obj.get_mut("paths").unwrap();
    let path = paths_ary.as_array().unwrap().get(index).unwrap();
    let path = path.as_string().unwrap();
    // Don't forget the default list - never delete that!
    if !path.contains("/Ananke/default-list.txt") {
        // Remove the button states
        if index < env.ui_state.header.file_menu_dynamic_forget_buttons.len() {
            env.ui_state
                .header
                .file_menu_dynamic_forget_buttons
                .remove(index);
        }
        if index < env.ui_state.header.file_menu_dynamic_load_buttons.len() {
            env.ui_state
                .header
                .file_menu_dynamic_load_buttons
                .remove(index);
        }

        // Remove the path
        let paths_ary = paths_ary.as_array_mut().unwrap();
        paths_ary.remove(index);
        debug_assert!(paths_ary.len() > 0);

        // Update the environment & config
        env.list = List::load(paths_ary.get(0).unwrap().as_string().unwrap()).unwrap();
        env.path_amount -= 1;
        let _ = env
            .disk_env
            .brigid
            .update_file("config.xff", Content::XFF(xff!(obj.clone())));
        env.render_tasks = env.list.tasks();
        env.ui_state.reset_menu(codex);
    }
}

/// Handles the mouse events for the load button
fn handle_mouse_load_button(name: &str, env: &mut Environment, obj: &mut Object, codex: &Codex) {
    // Close the load menu
    env.ui_state.header.file_menu_sub_load_button.clicked = false;

    // Save the current list
    let _ = env.list.save().unwrap();

    // Construct the path
    let index = name.split("_").last().unwrap().parse::<usize>().unwrap();
    let path = obj.get("paths").unwrap();
    let path = path.as_array().unwrap().get(index).unwrap();
    let path = path.as_string().unwrap();

    // Load the new list if it's different and update the config
    if env.list.get_path() != path {
        env.list = List::load(path).unwrap();
    }
    let _ = env
        .disk_env
        .brigid
        .update_file("config.xff", Content::XFF(xff!(obj.clone())));
    env.render_tasks = env.list.tasks();
    env.ui_state.reset_menu(codex);
}
