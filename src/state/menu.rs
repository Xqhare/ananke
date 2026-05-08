use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{DropdownState, ListState, TextBoxState},
    },
};

pub struct MenuState {
    pub sort_dropdown: DropdownState,
    pub sort_prio_textbox: TextBoxState,
    pub search_textbox: TextBoxState,
    pub show_dropdown: DropdownState,
}

impl MenuState {
    pub fn get_textboxes_mut(&mut self) -> Vec<&mut TextBoxState> {
        vec![&mut self.sort_prio_textbox, &mut self.search_textbox]
    }
}

pub fn make_menu_state(codex: &Codex) -> MenuState {
    MenuState {
        sort_dropdown: make_menu_sort_menu_state(),
        sort_prio_textbox: make_menu_sort_prio_text_state(codex),
        search_textbox: make_menu_search_text_state(codex),
        show_dropdown: make_menu_show_state(),
    }
}

fn make_menu_show_state() -> DropdownState {
    let list = ListState {
        selected: Some(0),
        scroll_offset: 0,
    };
    DropdownState {
        expanded: false,
        list_state: list,
    }
}

fn make_menu_sort_menu_state() -> DropdownState {
    let list = ListState {
        selected: Some(0),
        scroll_offset: 0,
    };
    DropdownState {
        expanded: false,
        list_state: list,
    }
}

fn make_menu_sort_prio_text_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_center().align_vertically(),
    }
}

fn make_menu_search_text_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_vertically().align_center(),
    }
}
