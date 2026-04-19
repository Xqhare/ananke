use std::collections::BTreeMap;

use anansi::List;
use talos::{codex::Codex, widgets::stateful::States};

use crate::state::{
    creator::make_creator_state, header::make_header_state, list::make_list_table_state,
    menu::make_menu_state,
};

mod creator;
mod header;
mod list;
mod menu;

// There is an argument to be made for `'static` lifetime here - The state must be valid for the
// entire lifetime of the application anyways. But I want to be consistent with the rest of the
// codebase - and I think its best practice to not use `'static`?
pub fn make_state<'a>(
    path_amount: usize,
    list: &List,
    codex: &Codex,
) -> BTreeMap<String, States<'a>> {
    let mut out = BTreeMap::new();
    make_header_state(path_amount, codex, &mut out);
    make_creator_state(codex, &mut out);
    make_menu_state(codex, &mut out);
    make_list_table_state(list, codex, &mut out);
    out
}
