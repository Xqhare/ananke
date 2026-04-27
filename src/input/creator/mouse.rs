use anansi::Task;
use horae::Utc;
use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{CreatorFocus, Focus},
    startup::Environment,
};

pub fn handle_creator_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    match name {
        "creator_textbox_task" => {
            let state = env
                .states
                .get_mut("creator_task_entry_textbox_state")
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            Focus::Creator(CreatorFocus::Task)
        }
        "creator_textbox_prio" => {
            let state = env
                .states
                .get_mut("creator_task_prio_entry_textbox_state")
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            Focus::Creator(CreatorFocus::Priority)
        }
        "creator_textbox_inception" => {
            let state = env
                .states
                .get_mut("creator_task_creation_date_entry_textbox_state")
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            Focus::Creator(CreatorFocus::CreationDate)
        }
        "creator_button_reset_new_task" => {
            reset_creator(env, codex);
            Focus::None
        }
        "creator_button_add_new_task" => {
            update_creator_task(env);
            env.list.push_task(env.new_task.clone());
            env.list.save().unwrap();
            reset_creator(env, codex);
            Focus::None
        }
        _ => Focus::None,
    }
}

fn update_creator_task(env: &mut Environment) {
    let prio = {
        let tmp = env
            .states
            .get("creator_task_prio_entry_textbox_state")
            .unwrap()
            .as_text_box()
            .unwrap()
            .text
            .get_content();
        if tmp.chars().count() > 0 {
            Some(tmp.chars().next().unwrap().to_uppercase().next().unwrap())
        } else {
            None
        }
    };
    let creation_date = env
        .states
        .get("creator_task_creation_date_entry_textbox_state")
        .unwrap()
        .as_text_box()
        .unwrap()
        .text
        .get_content();
    let text = env
        .states
        .get("creator_task_entry_textbox_state")
        .unwrap()
        .as_text_box()
        .unwrap()
        .text
        .get_content();
    env.new_task.update_text(text);
    if let Some(prio) = prio {
        env.new_task.update_prio(prio);
    }
    env.new_task.update_inception_date(creation_date);
}

fn reset_creator(env: &mut Environment, codex: &Codex) {
    env.new_task = Task::new("", env.list.max_id());
    env.states
        .get_mut("creator_task_entry_textbox_state")
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
    env.states
        .get_mut("creator_task_prio_entry_textbox_state")
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
    let mut now = Utc::now();
    now.with_auto_offset();
    env.states
        .get_mut("creator_task_creation_date_entry_textbox_state")
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content(&now.date().to_string(), codex);
}
