use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{CreatorFocus, header::handle_key_textbox_newfile},
    keys::{
        CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE, CREATOR_PRIO_ENTRY_TEXTBOX_STATE,
        CREATOR_TASK_ENTRY_TEXTBOX_STATE,
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
