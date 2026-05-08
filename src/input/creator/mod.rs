use anansi::{
    SortBy,
    vec::{search_vec_task_prio, search_vec_task_text, sort_vec_task},
};
use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::header::handle_key_textbox_newfile,
    startup::Environment,
    state::{CreatorFocus, Focus},
};

pub mod mouse;

pub fn handle_key_creator(
    key_event: &KeyEvent,
    env: &mut Environment,
    focus: &CreatorFocus,
    codex: &Codex,
) -> Option<()> {
    handle_key_textbox_newfile(key_event, env, codex, &Focus::Creator(*focus))
}

pub fn update_render_list(env: &mut Environment) {
    let show_state = env
        .ui_state
        .menu
        .show_dropdown
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
        .ui_state
        .menu
        .sort_dropdown
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

    let search_prio = env.ui_state.menu.sort_prio_textbox.text.get_content();
    env.render_tasks = search_vec_task_prio(&env.render_tasks, search_prio);

    let search_text = env.ui_state.menu.search_textbox.text.get_content();
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
    env.ui_state
        .creator
        .context_tags_text
        .text
        .set_content(context, codex);

    let project = env
        .new_task
        .projects()
        .iter()
        .map(|s| format!("+{s}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.ui_state
        .creator
        .project_tags_text
        .text
        .set_content(project, codex);

    let special = env
        .new_task
        .specials()
        .iter()
        .map(|(k, v)| format!("{k}:{v}"))
        .collect::<Vec<_>>()
        .join(", ");
    env.ui_state
        .creator
        .special_tags_text
        .text
        .set_content(special, codex);
}
