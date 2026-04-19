use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
};

pub fn make_menu_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    make_menu_sort_menu_state(out);
    make_menu_sort_prio_text_state(codex, out);
    make_menu_search_text_state(codex, out);
}

fn make_menu_sort_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        "make_menu_sort_menu_done_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
    out.insert(
        "make_menu_sort_menu_open_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
    out.insert(
        "make_menu_sort_menu_completion_date_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
    out.insert(
        "make_menu_sort_menu_inception_date_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
}

fn make_menu_sort_prio_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "make_menu_sort_prio_text_state".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex),
        }),
    );
}

fn make_menu_search_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "make_menu_search_text_state".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex),
        }),
    );
}
