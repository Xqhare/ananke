use persistent_state::PersistentState;

mod startup_state;
pub mod persistent_state;

pub struct State {
    pub persistent_state: PersistentState,
}

impl State {
    pub fn new(persistent_state: PersistentState) -> State {
        State {
            persistent_state,
        }
    }
}
