use crate::error::AnankeError;

use super::persistent_state::PersistentState;

pub struct StartupState {
    pub persistent_state: PersistentState,
    pub error: Option<AnankeError>,
    pub first_run: bool,
}

impl StartupState {
    pub fn new(persistent_state: PersistentState, first_run: bool, error: Option<AnankeError>) -> StartupState {
        StartupState {
            persistent_state,
            first_run,
            error,
        }
    }
}
