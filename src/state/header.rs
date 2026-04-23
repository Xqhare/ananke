use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
};

pub fn make_header_state(path_amount: usize, codex: &Codex, out: &mut BTreeMap<String, States>) {
    make_header_file_menu_state(path_amount, codex, out);
    make_header_save_menu_state(out);
    make_header_help_menu_state(out);
    make_header_exit_menu_state(out);
}

fn make_header_save_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        "header_save_menu_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
}
fn make_header_help_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        "header_help_menu_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
}
fn make_header_exit_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        "header_exit_menu_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
}

fn make_header_file_menu_state(
    path_amount: usize,
    codex: &Codex,
    out: &mut BTreeMap<String, States>,
) {
    let button_state = ButtonState { clicked: false };
    let text = Text::new("path_to/file.txt", codex)
        .align_center()
        .align_vertically();
    let path_state = TextBoxState {
        active: false,
        cursor: Some(text.len().saturating_sub(1)),
        text,
    };
    out.insert(
        "header_file_menu_button_main_button_state".to_string(),
        States::from(button_state),
    );
    out.insert(
        "header_file_menu_sub_new_button_state".to_string(),
        States::from(button_state),
    );
    out.insert(
        "header_file_menu_sub_new_textbox_state".to_string(),
        States::from(path_state.clone()),
    );
    out.insert(
        "header_file_menu_sub_load_button_state".to_string(),
        States::from(button_state),
    );
    out.insert(
        "header_file_menu_sub_forget_button_state".to_string(),
        States::from(button_state),
    );
    for i in 0..path_amount {
        out.insert(
            format!("header_file_menu_sub_forget_button_{i}"),
            States::from(button_state),
        );
        out.insert(
            format!("header_file_menu_sub_load_button_{i}"),
            States::from(button_state),
        );
    }
}
