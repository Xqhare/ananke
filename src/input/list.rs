use anansi::Date;
use horae::Utc;
use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::creator::update_render_list,
    keys::{
        LIST_ROW_COMPLETION_BASE, LIST_ROW_DELETE_BUTTON_BASE, LIST_ROW_DONE_BUTTON_BASE,
        LIST_ROW_INCEPTION_BASE, LIST_ROW_PRIO_BASE, LIST_ROW_TEXT_BASE,
    },
    startup::Environment,
    state::{Focus, ListFocus},
};

pub fn handle_list_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    let id = name.split("_").last().unwrap().parse::<usize>().unwrap();
    if name.starts_with(LIST_ROW_DONE_BUTTON_BASE) {
        let mut task = env.list.get(id).unwrap().clone();
        if task.is_done() {
            task = task.undone();
        } else {
            let mut date = Utc::now();
            date.with_auto_offset();
            task = task
                .done(Some(Date::from(date.date().to_string())))
                .unwrap();
        }

        env.list.update_task(task, id).unwrap();
        env.ui_state
            .dynamic_states
            .get_mut(&id)
            .unwrap()
            .done_button
            .clicked = !env
            .ui_state
            .dynamic_states
            .get_mut(&id)
            .unwrap()
            .done_button
            .clicked;

        update_render_list(env);

        Focus::None
    } else if name.starts_with(LIST_ROW_DELETE_BUTTON_BASE) {
        Focus::None
    } else if name.starts_with(LIST_ROW_PRIO_BASE) {
        env.ui_state
            .dynamic_states
            .get_mut(&id)
            .unwrap()
            .prio_textbox
            .active = true;
        Focus::List(ListFocus::Priority(id))
    } else if name.starts_with(LIST_ROW_INCEPTION_BASE) {
        env.ui_state
            .dynamic_states
            .get_mut(&id)
            .unwrap()
            .inception_textbox
            .active = true;
        Focus::List(ListFocus::Inception(id))
    } else if name.starts_with(LIST_ROW_COMPLETION_BASE) {
        env.ui_state
            .dynamic_states
            .get_mut(&id)
            .unwrap()
            .completion_textbox
            .active = true;
        Focus::List(ListFocus::Completion(id))
    } else if name.starts_with(LIST_ROW_TEXT_BASE) {
        env.ui_state
            .dynamic_states
            .get_mut(&id)
            .unwrap()
            .text_textbox
            .active = true;
        Focus::List(ListFocus::Task(id))
    } else {
        Focus::None
    }
}

pub fn handle_key_list(
    key_event: &KeyEvent,
    env: &mut Environment,
    name: &ListFocus,
    codex: &Codex,
) -> Option<()> {
    None
}
