use std::{collections::BTreeMap, path::PathBuf};

use anansi::List;
use talos::{
    codex::Codex,
    widgets::stateful::{ListState, States},
};

use crate::state::{
    creator::{CreatorState, make_creator_state},
    header::{HeaderState, make_header_state},
    list::make_list_table_state,
    menu::{MenuState, make_menu_state},
};

pub mod creator;
pub mod header;
pub mod list;
pub mod menu;

pub struct UiState<'a> {
    pub header: HeaderState,
    pub creator: CreatorState,
    pub menu: MenuState,
    pub list: ListState,
    // TODO: Refactor into a more generic solution: a map of `TaskState` structs, each holding
    // everything needed to render a task
    //
    // Also, the id of the task is the key -> the key could also now just be a usize instead
    pub dynamic_states: BTreeMap<String, States<'a>>,
}

// There is an argument to be made for `'static` lifetime here - The state must be valid for the
// entire lifetime of the application anyways. But I want to be consistent with the rest of the
// codebase - and I think its best practice to not use `'static`?
pub fn make_state<'a>(
    path_amount: usize,
    list: &List,
    codex: &Codex,
    home: &PathBuf,
) -> UiState<'a> {
    let mut dynamic_states = BTreeMap::new();
    let header = make_header_state(path_amount, codex, home);
    let creator = make_creator_state(codex);
    let menu = make_menu_state(codex);
    let list = make_list_table_state(list, codex, &mut dynamic_states);

    UiState {
        header,
        creator,
        menu,
        list,
        dynamic_states,
    }
}
