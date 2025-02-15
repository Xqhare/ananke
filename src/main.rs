use gui::gui_startup;
use state::persistent_state::PersistentState;

mod gui;
mod util;
mod state;

fn main() {
    gui_startup(PersistentState::default());
}
