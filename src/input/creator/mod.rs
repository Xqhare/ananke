use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{CreatorFocus, header::handle_key_textbox_newfile},
    keys::{
        CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE, CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
        CREATOR_TASK_ENTRY_TEXTBOX_STATE, CREATOR_TEXT_CONTEXT_TAGS, CREATOR_TEXT_PROJECT_TAGS,
        CREATOR_TEXT_SPECIAL_TAGS,
    },
    startup::Environment,
};

pub mod mouse;

pub fn handle_key_creator(
    key_event: &KeyEvent,
    env: &mut Environment,
    focus: &CreatorFocus,
    codex: &Codex,
) -> Option<()> {
    let name = match focus {
        CreatorFocus::Task => CREATOR_TASK_ENTRY_TEXTBOX_STATE,
        CreatorFocus::Priority => CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
        CreatorFocus::CreationDate => CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE,
    };

    handle_key_textbox_newfile(name, key_event, env, codex)
}

pub fn update_new_task_after_key_press(env: &mut Environment, name: &str, codex: &Codex) {
    let context = env
        .new_task
        .contexts()
        .iter()
        .map(|s| format!("@{s}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.states
        .get_mut(CREATOR_TEXT_CONTEXT_TAGS)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content(context, codex);

    let project = env
        .new_task
        .projects()
        .iter()
        .map(|s| format!("+{s}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.states
        .get_mut(CREATOR_TEXT_PROJECT_TAGS)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content(project, codex);

    let special = env
        .new_task
        .specials()
        .iter()
        .map(|(k, v)| format!("{k}:{v}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.states
        .get_mut(CREATOR_TEXT_SPECIAL_TAGS)
        .unwrap()
        .as_text_box_mut()
        .unwrap()
        .text
        .set_content(special, codex);
}
