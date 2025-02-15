use anansi::List;

use crate::state::{persistent_state::PersistentState, State};

pub struct Ananke {
    pub state: State,
    pub entire_list: List,
}


fn get_app_name() -> String {
    let mut app_name = env!("CARGO_PKG_NAME").to_string();
    app_name.remove(0);
    app_name.insert(0, app_name.chars().next().unwrap().to_ascii_uppercase());
    app_name
}

pub fn gui_startup(persistent_state: PersistentState) {
    let app_name = get_app_name();
    let state = State::new(persistent_state);
}
