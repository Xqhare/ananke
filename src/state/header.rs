use std::path::PathBuf;

use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, TextBoxState},
    },
};

pub struct HeaderState {
    pub file_menu_button: ButtonState,
    pub file_menu_sub_new_button: ButtonState,
    pub file_menu_sub_new_textbox: TextBoxState,
    pub file_menu_sub_load_button: ButtonState,
    pub file_menu_sub_forget_button: ButtonState,
    pub file_menu_dynamic_load_buttons: Vec<ButtonState>,
    pub file_menu_dynamic_forget_buttons: Vec<ButtonState>,
    pub save_button: ButtonState,
    pub help_button: ButtonState,
    pub exit_button: ButtonState,
}

pub fn make_header_state(path_amount: usize, codex: &Codex, home: &PathBuf) -> HeaderState {
    let mut state = HeaderState {
        file_menu_button: ButtonState { clicked: false },
        file_menu_sub_new_button: ButtonState { clicked: false },
        file_menu_sub_new_textbox: make_header_file_menu_sub_new_textbox_state(codex, home),
        file_menu_sub_load_button: ButtonState { clicked: false },
        file_menu_sub_forget_button: ButtonState { clicked: false },
        file_menu_dynamic_load_buttons: Vec::with_capacity(path_amount),
        file_menu_dynamic_forget_buttons: Vec::with_capacity(path_amount),
        save_button: ButtonState { clicked: false },
        help_button: ButtonState { clicked: false },
        exit_button: ButtonState { clicked: false },
    };

    for _ in 0..path_amount {
        state
            .file_menu_dynamic_load_buttons
            .push(ButtonState { clicked: false });
        state
            .file_menu_dynamic_forget_buttons
            .push(ButtonState { clicked: false });
    }

    state
}

fn make_header_file_menu_sub_new_textbox_state(codex: &Codex, home: &PathBuf) -> TextBoxState {
    let text = Text::new(home.to_string_lossy(), codex)
        .align_center()
        .align_vertically();
    TextBoxState {
        active: false,
        cursor: Some(text.len()),
        text,
    }
}
