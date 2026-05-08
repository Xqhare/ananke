use anansi::{
    SortBy,
    vec::{search_vec_task_prio, search_vec_task_text, sort_vec_task},
};
use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{handle_generic_textbox_input, header::keep_textbox_at_one_char},
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
    if handle_generic_textbox_input(
        key_event,
        env.ui_state.active_textbox_mut(&Focus::Creator(*focus))?,
        codex,
    ) {
        match focus {
            CreatorFocus::Task => {
                env.new_task
                    .update_text(env.ui_state.creator.task_entry_textbox.text.get_content());
                update_new_task_after_key_press(env, codex);
            }
            CreatorFocus::Priority => {
                if keep_textbox_at_one_char(env, &Focus::Creator(*focus), codex) {
                    env.ui_state.creator.prio_entry_textbox.active = false;
                    return Some(());
                }
            }
            CreatorFocus::CreationDate => {
                env.new_task.update_inception_date(
                    env.ui_state
                        .creator
                        .creation_date_entry_textbox
                        .text
                        .get_content(),
                );
            }
        }
    }

    None
}

pub fn update_render_list(env: &mut Environment) {
    match env
        .ui_state
        .menu
        .show_dropdown
        .list_state
        .selected
        .unwrap_or(0) // Default to show all; should be unreachable though
    {
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

    match env
        .ui_state
        .menu
        .sort_dropdown
        .list_state
        .selected
        .unwrap_or(0) // Default to not sort; should be unreachable though
    {
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

    env.render_tasks = search_vec_task_prio(
        &env.render_tasks,
        env.ui_state.menu.sort_prio_textbox.text.get_content(),
    );

    env.render_tasks = search_vec_task_text(
        &env.render_tasks,
        env.ui_state.menu.search_textbox.text.get_content(),
    );
}

pub fn update_new_task_after_key_press(env: &mut Environment, codex: &Codex) {
    env.ui_state.creator.context_tags_text.text.set_content(
        env.new_task
            .contexts()
            .iter()
            .map(|s| format!("@{s}"))
            .collect::<Vec<_>>()
            .join(", "),
        codex,
    );

    env.ui_state.creator.project_tags_text.text.set_content(
        env.new_task
            .projects()
            .iter()
            .map(|s| format!("+{s}"))
            .collect::<Vec<_>>()
            .join(", "),
        codex,
    );

    env.ui_state.creator.special_tags_text.text.set_content(
        env.new_task
            .specials()
            .iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect::<Vec<_>>()
            .join(", "),
        codex,
    );
}
