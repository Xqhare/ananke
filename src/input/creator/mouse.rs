use anansi::Task;
use horae::Utc;
use talos::codex::Codex;

use crate::{
    input::{CreatorFocus, Focus},
    keys::{
        CREATOR_CLEAR_BUTTON, CREATOR_INCEPTION_ENTRY_TEXTBOX, CREATOR_PRIO_ENTRY_TEXTBOX,
        CREATOR_SAVE_BUTTON, CREATOR_TASK_ENTRY_TEXTBOX,
    },
    startup::Environment,
};

pub fn handle_creator_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    match name {
        CREATOR_TASK_ENTRY_TEXTBOX => {
            env.ui_state.creator.task_entry_textbox.active = true;
            Focus::Creator(CreatorFocus::Task)
        }
        CREATOR_PRIO_ENTRY_TEXTBOX => {
            env.ui_state.creator.prio_entry_textbox.active = true;
            env.ui_state
                .creator
                .prio_entry_textbox
                .text
                .set_content("", codex);
            Focus::Creator(CreatorFocus::Priority)
        }
        CREATOR_INCEPTION_ENTRY_TEXTBOX => {
            env.ui_state.creator.creation_date_entry_textbox.active = true;
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
        let tmp = env.ui_state.creator.prio_entry_textbox.text.get_content();
        if tmp.chars().count() > 0 {
            Some(tmp.chars().next().unwrap().to_uppercase().next().unwrap())
        } else {
            None
        }
    };
    let creation_date = env
        .ui_state
        .creator
        .creation_date_entry_textbox
        .text
        .get_content();
    let text = env.ui_state.creator.task_entry_textbox.text.get_content();
    env.new_task.update_text(text);
    if let Some(prio) = prio {
        env.new_task.update_prio(prio);
    }
    env.new_task.update_inception_date(creation_date);
}

fn reset_creator(env: &mut Environment, codex: &Codex) {
    env.new_task = Task::new("", env.list.max_id());
    env.ui_state
        .creator
        .task_entry_textbox
        .text
        .set_content("", codex);
    env.ui_state
        .creator
        .prio_entry_textbox
        .text
        .set_content("", codex);
    let mut now = Utc::now();
    now.with_auto_offset();
    env.ui_state
        .creator
        .creation_date_entry_textbox
        .text
        .set_content(&now.date().to_string(), codex);

    env.ui_state
        .creator
        .context_tags_text
        .text
        .set_content("", codex);
    env.ui_state
        .creator
        .project_tags_text
        .text
        .set_content("", codex);
    env.ui_state
        .creator
        .special_tags_text
        .text
        .set_content("", codex);
}
