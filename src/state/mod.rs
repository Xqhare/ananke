//! # State Management & Focus Control
//!
//! This module defines the application's state architecture, demonstrating how **Talos**
//! separates persistent data (the Todo list) from transient UI state (cursor positions,
//! active textboxes, and focus).
//!
//! Talos uses **Stateful Widgets**. A widget's visual representation is separate from its
//! internal state, allowing the state to persist across frames while the layout and
//! canvas are rebuilt.

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

/// Represents the global 'Focus' of the application.
///
/// Because terminal input is a single stream of events, focus is used to route 
/// keyboard events to the correct stateful widget (e.g., which TextBox is receiving text).
#[derive(Clone, Copy, Debug)]
pub enum Focus {
    /// No interactive element is active.
    None,
    /// The 'New File' textbox in the header has focus.
    HeaderFileNewTextBox,
    /// A specific field in the task creator has focus.
    Creator(CreatorFocus),
    /// A search or filter field in the menu has focus.
    Menu(MenuFocus),
    /// A specific cell in the task list table has focus.
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

/// The aggregated UI state of Ananke.
pub struct UiState {
    /// State for the header (file menus, buttons).
    pub header: HeaderState,
    /// State for the task creator (entry fields).
    pub creator: CreatorState,
    /// State for the menu (dropdowns, search).
    pub menu: MenuState,
    /// Global state for the task table (scrolling, selection).
    pub task_table: TableState,
    /// Per-row dynamic states for tasks in the list.
    /// This demonstrates how to manage state for a dynamic number of widgets.
    pub dynamic_states: BTreeMap<usize, TaskState>,
}

impl UiState {
    pub fn reset_menu(&mut self, codex: &Codex) {
        self.menu = make_menu_state(codex);
    }

    /// Updates which widget has active focus.
    /// This method ensures that only one TextBox is 'active' at a time, which
    /// typically triggers a different visual style (e.g., a blinking cursor).
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

    /// Retrieves a mutable reference to the currently focused TextBoxState, if any.
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
        for state in self.dynamic_states.values_mut() {
            state.deactivate_all_textboxes();
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
