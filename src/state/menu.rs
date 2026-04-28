use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ListState, States, TextBoxState},
    },
};

use crate::keys::{
    MENU_SEARCH_TEXTBOX_STATE, MENU_SHOW_DROPDOWN_STATE, MENU_SORT_BUTTON_STATE,
    MENU_SORT_PRIO_TEXTBOX_STATE,
};

pub fn make_menu_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    make_menu_sort_menu_state(out);
    make_menu_sort_prio_text_state(codex, out);
    make_menu_search_text_state(codex, out);
    make_menu_show_state(out);
}

fn make_menu_show_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        MENU_SHOW_DROPDOWN_STATE.to_string(),
        States::from(ListState {
            selected: None,
            scroll_offset: 0,
        }),
    );
}

fn make_menu_sort_menu_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        MENU_SORT_BUTTON_STATE.to_string(),
        States::from(ListState {
            selected: None,
            scroll_offset: 0,
        }),
    );
}

fn make_menu_sort_prio_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        MENU_SORT_PRIO_TEXTBOX_STATE.to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex),
        }),
    );
}

fn make_menu_search_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        MENU_SEARCH_TEXTBOX_STATE.to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex),
        }),
    );
}
