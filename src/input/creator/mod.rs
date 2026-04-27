use anansi::Task;
use horae::Utc;
use talos::{
    codex::{self, Codex},
    input::KeyEvent,
};

use crate::{
    input::{CreatorFocus, Focus, header::handle_key_textbox_newfile},
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
        CreatorFocus::Task => "creator_task_entry_textbox_state",
        CreatorFocus::Priority => "creator_task_prio_entry_textbox_state",
        CreatorFocus::CreationDate => "creator_task_creation_date_entry_textbox_state",
    };

    handle_key_textbox_newfile(name, key_event, env, codex)
}
