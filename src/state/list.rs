use std::collections::BTreeMap;

use anansi::{List, Task};
use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
};

use crate::keys::{
    LIST_SINGLE_TASK_DELETE_BUTTON_STATE, LIST_SINGLE_TASK_DONE_BUTTON_STATE,
    LIST_SINGLE_TASK_PRIO_TEXTBOX_STATE, LIST_SINGLE_TASK_TEXT_TEXTBOX_STATE,
};

pub fn make_list_table_state(list: &List, codex: &Codex, out: &mut BTreeMap<String, States>) {
    for task in &list.tasks() {
        make_single_task_state(task, codex, out);
    }
}

// TODO: Will need to call that when updating a task.
/// Creates the state for a single task
pub fn make_single_task_state(task: &Task, codex: &Codex, out: &mut BTreeMap<String, States>) {
    let id = task.id();
    let task_status = task.is_done();
    out.insert(
        format!("{LIST_SINGLE_TASK_DONE_BUTTON_STATE}{id}").to_string(),
        ButtonState {
            clicked: task_status,
        }
        .into(),
    );
    out.insert(
        format!("{LIST_SINGLE_TASK_DELETE_BUTTON_STATE}{id}").to_string(),
        ButtonState { clicked: false }.into(),
    );
    let task_priority = task.prio().unwrap_or(' ');
    out.insert(
        format!("{LIST_SINGLE_TASK_PRIO_TEXTBOX_STATE}{id}").to_string(),
        TextBoxState {
            active: false,
            cursor: None,
            text: Text::new(task_priority, codex),
        }
        .into(),
    );
    let task_text = task.text();
    out.insert(
        format!("{LIST_SINGLE_TASK_TEXT_TEXTBOX_STATE}{id}").to_string(),
        TextBoxState {
            active: false,
            cursor: None,
            text: Text::new(task_text, codex),
        }
        .into(),
    );
}
