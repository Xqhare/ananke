use anansi::{
    SortBy,
    vec::{search_vec_task_prio, search_vec_task_text, sort_vec_task},
};
use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{CreatorFocus, header::handle_key_textbox_newfile},
    keys::{
        CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE, CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
        CREATOR_TASK_ENTRY_TEXTBOX_STATE, CREATOR_TEXT_CONTEXT_TAGS, CREATOR_TEXT_PROJECT_TAGS,
        CREATOR_TEXT_SPECIAL_TAGS, MENU_SEARCH_PRIO_TEXTBOX_STATE, MENU_SEARCH_TEXTBOX_STATE,
        MENU_SHOW_DROPDOWN, MENU_SHOW_DROPDOWN_STATE, MENU_SORT_DROPDOWN, MENU_SORT_DROPDOWN_STATE,
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

pub fn update_render_list(env: &mut Environment) {
    let show_state = env
        .states
        .get(MENU_SHOW_DROPDOWN_STATE)
        .unwrap()
        .as_dropdown()
        .unwrap()
        .list_state
        .selected
        .unwrap_or(0); // Default to show all; should be unreachable though

    match show_state {
        0 => {
            env.render_tasks = env.list.tasks();
        }
        1 => {
            env.render_tasks = env.list.done();
        }
        2 => {
            env.render_tasks = env.list.open();
        }
        _ => unreachable!(),
    }

    let sort_state = env
        .states
        .get(MENU_SORT_DROPDOWN_STATE)
        .unwrap()
        .as_dropdown()
        .unwrap()
        .list_state
        .selected
        .unwrap_or(0); // Default to not sort; should be unreachable though

    match sort_state {
        // No sort
        0 => {}
        1 => {
            sort_vec_task(&mut env.render_tasks, SortBy::Priority);
        }
        2 => {
            sort_vec_task(&mut env.render_tasks, SortBy::InceptionDate);
        }
        3 => {
            sort_vec_task(&mut env.render_tasks, SortBy::CompletionDate);
        }
        _ => unreachable!(),
    }

    let search_prio = env
        .states
        .get(MENU_SEARCH_PRIO_TEXTBOX_STATE)
        .unwrap()
        .as_text_box()
        .unwrap()
        .text
        .get_content();
    env.render_tasks = search_vec_task_prio(&env.render_tasks, search_prio);

    let search_text = env
        .states
        .get(MENU_SEARCH_TEXTBOX_STATE)
        .unwrap()
        .as_text_box()
        .unwrap()
        .text
        .get_content();
    env.render_tasks = search_vec_task_text(&env.render_tasks, search_text);
}

pub fn update_new_task_after_key_press(env: &mut Environment, codex: &Codex) {
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
