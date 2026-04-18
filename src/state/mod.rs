use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, States, TextBoxState},
    },
};

use crate::{
    error::{AnankeError, AnankeResult},
    startup::State,
    state::{creator::make_creator_state, header::make_header_state},
};

mod creator;
mod header;

// There is an argument to be made for `'static` lifetime here - The state must be valid for the
// entire lifetime of the application anyways. But I want to be consistent with the rest of the
// codebase
pub fn make_state<'a>(
    env: &State,
    path_amount: usize,
    codex: &Codex,
) -> BTreeMap<String, States<'a>> {
    let mut out = BTreeMap::new();
    make_header_state(path_amount, codex, &mut out);
    make_creator_state(codex, &mut out);
    out
}
