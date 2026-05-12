use std::{collections::BTreeMap, path::PathBuf};

use anansi::List;
use talos::{
    codex::Codex,
    widgets::stateful::{TableState, TextBoxState},
};

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
    /// Focus on some part of the list
    List(ListFocus),
}

/// The focus of the list
///
/// All members wrap the id of the task
#[derive(Clone, Copy, Debug)]
pub enum ListFocus {
    /// Focus on the task table priority textbox
    Priority(usize),
    /// Focus on the task table inception date textbox
    Inception(usize),
    /// Focus on the task table completion date textbox
    Completion(usize),
    /// Focus on the task table text textbox
    Task(usize),
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
    pub task_table: TableState,
    pub dynamic_states: BTreeMap<usize, TaskState>,
}

impl UiState {
    pub fn reset_menu(&mut self, codex: &Codex) {
        self.menu = make_menu_state(codex);
    }
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
            Focus::List(list) => match list {
                ListFocus::Priority(id) => {
                    self.dynamic_states
                        .get_mut(&id)
                        .unwrap()
                        .prio_textbox
                        .active = true;
                }
                ListFocus::Inception(id) => {
                    self.dynamic_states
                        .get_mut(&id)
                        .unwrap()
                        .inception_textbox
                        .active = true;
                }
                ListFocus::Completion(id) => {
                    self.dynamic_states
                        .get_mut(&id)
                        .unwrap()
                        .completion_textbox
                        .active = true;
                }
                ListFocus::Task(id) => {
                    self.dynamic_states
                        .get_mut(&id)
                        .unwrap()
                        .text_textbox
                        .active = true;
                }
            },
        }
    }

    pub fn active_textbox_mut(&mut self, focus: &Focus) -> Option<&mut TextBoxState> {
        match focus {
            Focus::None => None,
            Focus::HeaderFileNewTextBox => Some(&mut self.header.file_menu_sub_new_textbox),
            Focus::Creator(CreatorFocus::Task) => Some(&mut self.creator.task_entry_textbox),
            Focus::Creator(CreatorFocus::Priority) => Some(&mut self.creator.prio_entry_textbox),
            Focus::Creator(CreatorFocus::CreationDate) => {
                Some(&mut self.creator.creation_date_entry_textbox)
            }
            Focus::Menu(MenuFocus::Text) => Some(&mut self.menu.search_textbox),
            Focus::Menu(MenuFocus::Priority) => Some(&mut self.menu.sort_prio_textbox),
            Focus::List(list) => match list {
                ListFocus::Priority(id) => {
                    Some(&mut self.dynamic_states.get_mut(&id).unwrap().prio_textbox)
                }
                ListFocus::Inception(id) => {
                    Some(&mut self.dynamic_states.get_mut(&id).unwrap().inception_textbox)
                }
                ListFocus::Completion(id) => {
                    Some(&mut self.dynamic_states.get_mut(&id).unwrap().completion_textbox)
                }
                ListFocus::Task(id) => {
                    Some(&mut self.dynamic_states.get_mut(&id).unwrap().text_textbox)
                }
            },
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
        task_table: list,
        dynamic_states,
    }
}
