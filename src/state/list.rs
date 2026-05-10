use std::collections::BTreeMap;

use anansi::{List, Task};
use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, SequenceState, TableState, TextBoxState},
    },
};

pub struct TaskState {
    pub done_button: ButtonState,
    pub delete_button: ButtonState,
    pub prio_textbox: TextBoxState,
    pub text_textbox: TextBoxState,
    pub inception_textbox: TextBoxState,
    pub completion_textbox: TextBoxState,
    /// The state of all sequences, no special cases with no mutation
    pub generic_sequence: SequenceState,
}

pub fn make_list_table_state(
    list: &List,
    codex: &Codex,
    out: &mut BTreeMap<usize, TaskState>,
) -> TableState {
    for task in &list.tasks() {
        make_single_task_state(task, codex, out);
    }
    TableState {
        x_offset: 0,
        y_offset: 0,
        max_rows: None,
        max_columns: None,
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
        text: Text::new(task_priority, codex)
            .align_vertically()
            .align_center(),
    };
    let task_text = task.text();
    let text_textbox = TextBoxState {
        active: false,
        cursor: None,
        text: Text::new(task_text, codex)
            .align_vertically()
            .align_center(),
    };
    let generic_sequence = SequenceState { scroll_offset: 0 };
    let inception_text = task.inception_date();
    let inception_textbox = TextBoxState {
        active: false,
        cursor: None,
        text: Text::new(inception_text, codex)
            .align_center()
            .align_vertically(),
    };
    let completion_text = task.completion_date();
    let completion_textbox = TextBoxState {
        active: false,
        cursor: None,
        text: Text::new(completion_text, codex)
            .align_center()
            .align_vertically(),
    };

    out.insert(
        id,
        TaskState {
            done_button,
            delete_button,
            prio_textbox,
            text_textbox,
            generic_sequence,
            inception_textbox,
            completion_textbox,
        },
    );
}
