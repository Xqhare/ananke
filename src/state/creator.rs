use std::collections::BTreeMap;

use horae::Utc;
use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
};

pub fn make_creator_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    make_creator_task_entry_textbox_state(codex, out);
    make_creator_task_prio_entry_textbox_state(codex, out);
    make_creator_task_creation_date_entry_textbox_state(codex, out);
    make_creator_task_project_tags_text_state(codex, out);
    make_creator_task_context_tags_text_state(codex, out);
    make_creator_task_special_tags_text_state(codex, out);
    create_creator_task_forget_button_state(out);
    make_creator_task_save_button_state(out);
}

// TODO: Need for process management later, when save button is hit, recreate the default state for
// the creator
/// Creates the state for the task creation date entry
///
/// Polls the current date every time the function is called
pub fn make_creator_task_creation_date_entry_textbox_state(
    codex: &Codex,
    out: &mut BTreeMap<String, States>,
) {
    let mut now = Utc::now();
    // automatic time zone detection
    now.with_auto_offset();
    let str = now.date().to_string();
    out.insert(
        "creator_task_creation_date_entry_textbox_state".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(str.chars().count()),
            text: Text::new(str, codex).align_center(),
        }),
    );
}

fn make_creator_task_save_button_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_task_save_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
}

fn create_creator_task_forget_button_state(out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_task_forget_button_state".to_string(),
        States::from(ButtonState { clicked: false }),
    );
}

fn make_creator_task_prio_entry_textbox_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_task_prio_entry_textbox_state".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex).align_center(),
        }),
    );
}

fn make_creator_task_context_tags_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_text_context_tags".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex).align_center(),
        }),
    );
}

fn make_creator_task_project_tags_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_text_project_tags".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex).align_center(),
        }),
    );
}

fn make_creator_task_special_tags_text_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_text_special_tags".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex).align_center(),
        }),
    );
}

fn make_creator_task_entry_textbox_state(codex: &Codex, out: &mut BTreeMap<String, States>) {
    out.insert(
        "creator_task_entry_textbox_state".to_string(),
        States::from(TextBoxState {
            active: false,
            cursor: Some(0),
            text: Text::new("", codex).align_center().align_vertically(),
        }),
    );
}
