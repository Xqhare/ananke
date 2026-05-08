use std::collections::BTreeMap;

use anansi::{List, Task};
use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, ListState, TextBoxState},
    },
};

pub struct TaskState {
    pub done_button: ButtonState,
    pub delete_button: ButtonState,
    pub prio_textbox: TextBoxState,
    pub text_textbox: TextBoxState,
}

pub fn make_list_table_state(
    list: &List,
    codex: &Codex,
    out: &mut BTreeMap<usize, TaskState>,
) -> ListState {
    for task in &list.tasks() {
        make_single_task_state(task, codex, out);
    }
    ListState {
        selected: None,
        scroll_offset: 0,
    }
}

// TODO: Will need to call that when updating a task.
/// Creates the state for a single task
pub fn make_single_task_state(task: &Task, codex: &Codex, out: &mut BTreeMap<usize, TaskState>) {
    let id = task.id();
    let task_status = task.is_done();
    let done_button = ButtonState {
        clicked: task_status,
    };
    let delete_button = ButtonState { clicked: false };
    let task_priority = task.prio().unwrap_or(' ');
    let prio_textbox = TextBoxState {
        active: false,
        cursor: None,
        text: Text::new(task_priority, codex),
    };
    let task_text = task.text();
    let text_textbox = TextBoxState {
        active: false,
        cursor: None,
        text: Text::new(task_text, codex),
    };

    out.insert(
        id,
        TaskState {
            done_button,
            delete_button,
            prio_textbox,
            text_textbox,
        },
    );
}
