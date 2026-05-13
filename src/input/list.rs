use anansi::Date;
use horae::Utc;
use talos::{
    codex::Codex,
    input::{KeyCode, KeyEvent},
};

use crate::{
    input::{
        creator::update_render_list, handle_generic_textbox_input, header::keep_textbox_at_one_char,
    },
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
            env.ui_state
                .dynamic_states
                .get_mut(&id)
                .unwrap()
                .completion_textbox
                .text
                .set_content("", codex);
        } else {
            let mut date = Utc::now();
            date.with_auto_offset();
            task = task
                .done(Some(Date::from(date.date().to_string())))
                .unwrap();
            env.ui_state
                .dynamic_states
                .get_mut(&id)
                .unwrap()
                .completion_textbox
                .text
                .set_content(date.date().to_string(), codex);
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
        let _ = env.list.remove(id);
        update_render_list(env);
        Focus::None
    } else if name.starts_with(LIST_ROW_PRIO_BASE) {
        let state = env.ui_state.dynamic_states.get_mut(&id).unwrap();
        state.prio_textbox.active = true;
        if state.prio_textbox.cursor.is_none() {
            state.prio_textbox.cursor = Some(state.prio_textbox.text.len());
        }
        Focus::List(ListFocus::Priority(id))
    } else if name.starts_with(LIST_ROW_INCEPTION_BASE) {
        let state = env.ui_state.dynamic_states.get_mut(&id).unwrap();
        state.inception_textbox.active = true;
        if state.inception_textbox.cursor.is_none() {
            state.inception_textbox.cursor = Some(state.inception_textbox.text.len());
        }
        Focus::List(ListFocus::Inception(id))
    } else if name.starts_with(LIST_ROW_COMPLETION_BASE) {
        let state = env.ui_state.dynamic_states.get_mut(&id).unwrap();
        state.completion_textbox.active = true;
        if state.completion_textbox.cursor.is_none() {
            state.completion_textbox.cursor = Some(state.completion_textbox.text.len());
        }
        Focus::List(ListFocus::Completion(id))
    } else if name.starts_with(LIST_ROW_TEXT_BASE) {
        let state = env.ui_state.dynamic_states.get_mut(&id).unwrap();
        state.text_textbox.active = true;
        if state.text_textbox.cursor.is_none() {
            state.text_textbox.cursor = Some(state.text_textbox.text.len());
        }
        Focus::List(ListFocus::Task(id))
    } else {
        Focus::None
    }
}

pub fn handle_key_list(
    key_event: &KeyEvent,
    env: &mut Environment,
    focus: &ListFocus,
    codex: &Codex,
) -> Option<()> {
    if key_event.code == KeyCode::Enter {
        return Some(());
    }

    let id = match focus {
        ListFocus::Task(id) => *id,
        ListFocus::Priority(id) => *id,
        ListFocus::Inception(id) => *id,
        ListFocus::Completion(id) => *id,
    };

    let changed = match focus {
        ListFocus::Task(_) => handle_generic_textbox_input(
            key_event,
            &mut env
                .ui_state
                .dynamic_states
                .get_mut(&id)
                .unwrap()
                .text_textbox,
            codex,
        ),
        ListFocus::Priority(_) => {
            if handle_generic_textbox_input(
                key_event,
                &mut env
                    .ui_state
                    .dynamic_states
                    .get_mut(&id)
                    .unwrap()
                    .prio_textbox,
                codex,
            ) {
                if keep_textbox_at_one_char(env, &Focus::List(ListFocus::Priority(id)), codex) {
                    env.ui_state
                        .dynamic_states
                        .get_mut(&id)
                        .unwrap()
                        .prio_textbox
                        .active = false;
                }
                true
            } else {
                false
            }
        }
        ListFocus::Inception(_) => handle_generic_textbox_input(
            key_event,
            &mut env
                .ui_state
                .dynamic_states
                .get_mut(&id)
                .unwrap()
                .inception_textbox,
            codex,
        ),
        ListFocus::Completion(_) => handle_generic_textbox_input(
            key_event,
            &mut env
                .ui_state
                .dynamic_states
                .get_mut(&id)
                .unwrap()
                .completion_textbox,
            codex,
        ),
    };

    if changed {
        let mut task = env.list.get(id).unwrap().clone();
        task.update_text(
            env.ui_state
                .dynamic_states
                .get(&id)
                .unwrap()
                .text_textbox
                .text
                .get_content()
                .to_string(),
        );
        let new_prio = env
            .ui_state
            .dynamic_states
            .get(&id)
            .unwrap()
            .prio_textbox
            .text
            .get_content()
            .chars();
        if new_prio.clone().count() > 0 {
            let char = new_prio.take(1).next().unwrap();
            if char != ' ' {
                task.update_prio(char);
            }
        }
        task.update_inception_date(
            env.ui_state
                .dynamic_states
                .get(&id)
                .unwrap()
                .inception_textbox
                .text
                .get_content(),
        );
        if task.is_done() {
            let _ = task.update_completion_date(
                env.ui_state
                    .dynamic_states
                    .get(&id)
                    .unwrap()
                    .completion_textbox
                    .text
                    .get_content(),
            );
        }
        env.list.update_task(task, id).unwrap();
        update_render_list(env);
        env.list.save().unwrap();
    }

    None
}
