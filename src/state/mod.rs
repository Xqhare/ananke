use persistent_state::PersistentState;

use crate::error::AnankeError;

pub mod startup_state;
pub mod persistent_state;

pub struct State {
    pub persistent_state: PersistentState,
    pub error: Option<AnankeError>,
}

impl State {
    pub fn new(persistent_state: PersistentState) -> State {
        State {
            persistent_state,
            error: None,
        }
    }
}
