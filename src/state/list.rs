use std::collections::BTreeMap;

use anansi::{List, Task};
use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
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
        format!("make_single_task_id_text_state_{id}").to_string(),
        ButtonState {
            clicked: task_status,
        }
        .into(),
    );
    out.insert(
        format!("make_single_task_text_state_{id}").to_string(),
        ButtonState { clicked: false }.into(),
    );
    let task_priority = task.prio().unwrap_or(' ');
    out.insert(
        format!("make_single_task_prio_text_state_{id}").to_string(),
        TextBoxState {
            active: false,
            cursor: None,
            text: Text::new(task_priority, codex),
        }
        .into(),
    );
    let task_text = task.text();
    out.insert(
        format!("make_single_task_text_state_{id}").to_string(),
        TextBoxState {
            active: false,
            cursor: None,
            text: Text::new(task_text, codex),
        }
        .into(),
    );
}
