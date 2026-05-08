use std::{collections::BTreeMap, path::PathBuf};

use anansi::List;
use talos::{codex::Codex, widgets::stateful::ListState};

use crate::state::{
    creator::{CreatorState, make_creator_state},
    header::{HeaderState, make_header_state},
    list::{TaskState, make_list_table_state},
    menu::{MenuState, make_menu_state},
};

pub mod creator;
pub mod header;
pub mod list;
pub mod menu;

/// The focus of the program
///
/// Used for keyboard input capture and ensuring that the focus is on the active textfield
#[derive(Clone, Copy, Debug)]
pub enum Focus {
    /// No focus
    None,
    /// Focus on the header new-file textbox
    HeaderFileNewTextBox,
    /// Focus on some part of the creator
    Creator(CreatorFocus),
    /// Focus on some part of the menu
    Menu(MenuFocus),
}

/// The focus of the creator
#[derive(Clone, Copy, Debug)]
pub enum CreatorFocus {
    /// Focus on the creator new-task textbox
    Task,
    /// Focus on the creator priority textbox
    Priority,
    /// Focus on the creator date textbox
    CreationDate,
}

/// The focus of the menu
#[derive(Clone, Copy, Debug)]
pub enum MenuFocus {
    /// Focus on the menu search priority textbox
    Priority,
    /// Focus on the menu search textbox for text and tags
    Text,
}

pub struct UiState {
    pub header: HeaderState,
    pub creator: CreatorState,
    pub menu: MenuState,
    pub list: ListState,
    pub dynamic_states: BTreeMap<usize, TaskState>,
}

impl UiState {
    pub fn set_focus(&mut self, focus: Focus) {
        self.deactivate_all_textboxes();

        match focus {
            Focus::None => {}
            Focus::HeaderFileNewTextBox => {
                self.header.file_menu_sub_new_textbox.active = true;
            }
            Focus::Creator(CreatorFocus::Task) => {
                self.creator.task_entry_textbox.active = true;
            }
            Focus::Creator(CreatorFocus::Priority) => {
                self.creator.prio_entry_textbox.active = true;
            }
            Focus::Creator(CreatorFocus::CreationDate) => {
                self.creator.creation_date_entry_textbox.active = true;
            }
            Focus::Menu(MenuFocus::Text) => {
                self.menu.search_textbox.active = true;
            }
            Focus::Menu(MenuFocus::Priority) => {
                self.menu.sort_prio_textbox.active = true;
            }
        }
    }

    fn deactivate_all_textboxes(&mut self) {
        for tb in self.header.get_textboxes_mut() {
            tb.active = false;
        }
        for tb in self.creator.get_textboxes_mut() {
            tb.active = false;
        }
        for tb in self.menu.get_textboxes_mut() {
            tb.active = false;
        }
    }
}

pub fn make_state(path_amount: usize, list: &List, codex: &Codex, home: &PathBuf) -> UiState {
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
