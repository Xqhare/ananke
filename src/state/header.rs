use std::{collections::BTreeMap, path::PathBuf};

use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
};

use crate::keys::{
    HEADER_EXIT_BUTTON_STATE, HEADER_FILE_MENU_BUTTON_STATE,
    HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE, HEADER_FILE_MENU_SUB_FORGET_BUTTON_STATE,
    HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE, HEADER_FILE_MENU_SUB_LOAD_BUTTON_STATE,
    HEADER_FILE_MENU_SUB_NEW_BUTTON_STATE, HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE,
    HEADER_HELP_BUTTON_STATE, HEADER_SAVE_BUTTON_STATE,
};

pub fn make_header_state(
    path_amount: usize,
    codex: &Codex,
    out: &mut BTreeMap<String, States>,
    home: &PathBuf,
) {
    make_header_file_menu_state(path_amount, codex, out, home);
    make_header_save_menu_state(out);
    make_header_help_menu_state(out);
    make_header_exit_menu_state(out);
}

fn make_header_save_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        HEADER_SAVE_BUTTON_STATE.to_string(),
        States::from(ButtonState { clicked: false }),
    );
}
fn make_header_help_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        HEADER_HELP_BUTTON_STATE.to_string(),
        States::from(ButtonState { clicked: false }),
    );
}
fn make_header_exit_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        HEADER_EXIT_BUTTON_STATE.to_string(),
        States::from(ButtonState { clicked: false }),
    );
}

fn make_header_file_menu_state(
    path_amount: usize,
    codex: &Codex,
    out: &mut BTreeMap<String, States>,
    home: &PathBuf,
) {
    let button_state = ButtonState { clicked: false };
    let text = Text::new(home.to_string_lossy(), codex)
        .align_center()
        .align_vertically();
    let path_state = TextBoxState {
        active: false,
        cursor: Some(text.len()),
        text,
    };
    out.insert(
        HEADER_FILE_MENU_BUTTON_STATE.to_string(),
        States::from(button_state),
    );
    out.insert(
        HEADER_FILE_MENU_SUB_NEW_BUTTON_STATE.to_string(),
        States::from(button_state),
    );
    out.insert(
        HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE.to_string(),
        States::from(path_state.clone()),
    );
    out.insert(
        HEADER_FILE_MENU_SUB_LOAD_BUTTON_STATE.to_string(),
        States::from(button_state),
    );
    out.insert(
        HEADER_FILE_MENU_SUB_FORGET_BUTTON_STATE.to_string(),
        States::from(button_state),
    );
    for i in 0..path_amount {
        out.insert(
            format!("{HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE}{i}"),
            States::from(button_state),
        );
        out.insert(
            format!("{HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE}{i}"),
            States::from(button_state),
        );
    }
}
