use std::fmt::Display;

use persistent_state::PersistentState;
use anansi::SortBy;

use crate::error::AnankeError;

pub mod startup_state;
pub mod persistent_state;

pub struct State {
    pub persistent_state: PersistentState,
    pub search_state: SearchState,
    pub error: Option<AnankeError>,
    pub gui_state: GuiState,
}

impl State {
    pub fn new(persistent_state: PersistentState) -> State {
        State {
            persistent_state,
            error: None,
            search_state: SearchState::default(),
            gui_state: GuiState::default(),
        }
    }
}

pub struct GuiState {
    pub editor_gui_state: EditorGuiState,
}

impl GuiState {
    pub fn default() -> GuiState {
        GuiState {
            editor_gui_state: EditorGuiState::default(),
        }
    }
}

pub struct EditorGuiState {
    pub edit_date: bool,
    pub confirm_reset: bool,
}

impl EditorGuiState {
    pub fn default() -> EditorGuiState {
        EditorGuiState {
            edit_date: false,
            confirm_reset: false,
        }
    }
}

pub struct SearchState {
    pub show: Show,
    pub sort_by: SortBy,
    pub search_text: String,
    pub search_priority: String,
    pub search_project: String,
    pub search_context: String,
    pub search_special: String,
}

impl SearchState {
    pub fn default() -> SearchState {
        SearchState {
            show: Show::Open,
            sort_by: SortBy::Priority,
            search_text: String::default(),
            search_priority: String::default(),
            search_project: String::default(),
            search_context: String::default(),
            search_special: String::default(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Show {
    All,
    Open,
    Done,
}

impl Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Show::All => write!(f, "All"),
            Show::Open => write!(f, "Open"),
            Show::Done => write!(f, "Done"),
        }
    }
}

impl Into<String> for Show {
    fn into(self) -> String {
        match self {
            Show::All => "All".to_string(),
            Show::Open => "Open".to_string(),
            Show::Done => "Done".to_string(),
        }
    }
}
