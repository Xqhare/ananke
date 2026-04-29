use anansi::Task;
use horae::Utc;
use talos::codex::Codex;

use crate::{
    input::{CreatorFocus, Focus},
    keys::{
        CREATOR_CLEAR_BUTTON, CREATOR_INCEPTION_ENTRY_TEXTBOX,
        CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE, CREATOR_PRIO_ENTRY_TEXTBOX,
        CREATOR_PRIO_ENTRY_TEXTBOX_STATE, CREATOR_SAVE_BUTTON, CREATOR_TASK_ENTRY_TEXTBOX,
        CREATOR_TASK_ENTRY_TEXTBOX_STATE, CREATOR_TEXT_CONTEXT_TAGS, CREATOR_TEXT_PROJECT_TAGS,
        CREATOR_TEXT_SPECIAL_TAGS,
    },
    startup::Environment,
};

pub fn handle_creator_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    match name {
        CREATOR_TASK_ENTRY_TEXTBOX => {
            let state = env
                .states
                .get_mut(CREATOR_TASK_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            Focus::Creator(CreatorFocus::Task)
        }
        CREATOR_PRIO_ENTRY_TEXTBOX => {
            let state = env
                .states
                .get_mut(CREATOR_PRIO_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            state.text.set_content("", codex);
            Focus::Creator(CreatorFocus::Priority)
        }
        CREATOR_INCEPTION_ENTRY_TEXTBOX => {
            let state = env
                .states
                .get_mut(CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            Focus::Creator(CreatorFocus::CreationDate)
        }
        CREATOR_CLEAR_BUTTON => {
            reset_creator(env, codex);
            Focus::None
        }
        CREATOR_SAVE_BUTTON => {
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
            .get(CREATOR_PRIO_ENTRY_TEXTBOX_STATE)
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
        .get(CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE)
        .unwrap()
        .as_text_box()
        .unwrap()
        .text
        .get_content();
    let text = env
        .states
        .get(CREATOR_TASK_ENTRY_TEXTBOX_STATE)
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
        .get_mut(CREATOR_TASK_ENTRY_TEXTBOX_STATE)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
    env.states
        .get_mut(CREATOR_PRIO_ENTRY_TEXTBOX_STATE)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
    let mut now = Utc::now();
    now.with_auto_offset();
    env.states
        .get_mut(CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content(&now.date().to_string(), codex);

    env.states
        .get_mut(CREATOR_TEXT_CONTEXT_TAGS)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
    env.states
        .get_mut(CREATOR_TEXT_PROJECT_TAGS)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
    env.states
        .get_mut(CREATOR_TEXT_SPECIAL_TAGS)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content("", codex);
}
