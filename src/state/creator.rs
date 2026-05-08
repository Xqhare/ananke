use horae::Utc;
use talos::{
    codex::Codex,
    widgets::{
        Text,
        stateful::{ButtonState, TextBoxState},
    },
};

pub struct CreatorState {
    pub task_entry_textbox: TextBoxState,
    pub prio_entry_textbox: TextBoxState,
    pub creation_date_entry_textbox: TextBoxState,
    pub project_tags_text: TextBoxState,
    pub context_tags_text: TextBoxState,
    pub special_tags_text: TextBoxState,
    pub clear_button: ButtonState,
    pub save_button: ButtonState,
}

impl CreatorState {
    pub fn get_textboxes_mut(&mut self) -> Vec<&mut TextBoxState> {
        vec![
            &mut self.task_entry_textbox,
            &mut self.prio_entry_textbox,
            &mut self.creation_date_entry_textbox,
            &mut self.project_tags_text,
            &mut self.context_tags_text,
            &mut self.special_tags_text,
        ]
    }

    pub fn reset(&mut self, codex: &Codex) {
        self.task_entry_textbox = make_creator_task_entry_textbox_state(codex);
        self.prio_entry_textbox = make_creator_task_prio_entry_textbox_state(codex);
        self.creation_date_entry_textbox =
            make_creator_task_creation_date_entry_textbox_state(codex);
        self.project_tags_text = make_creator_task_project_tags_text_state(codex);
        self.context_tags_text = make_creator_task_context_tags_text_state(codex);
        self.special_tags_text = make_creator_task_special_tags_text_state(codex);
        self.clear_button = create_creator_task_forget_button_state();
        self.save_button = make_creator_task_save_button_state();
    }
}

pub fn make_creator_state(codex: &Codex) -> CreatorState {
    let mut state = CreatorState {
        task_entry_textbox: TextBoxState::default(),
        prio_entry_textbox: TextBoxState::default(),
        creation_date_entry_textbox: TextBoxState::default(),
        project_tags_text: TextBoxState::default(),
        context_tags_text: TextBoxState::default(),
        special_tags_text: TextBoxState::default(),
        clear_button: ButtonState::default(),
        save_button: ButtonState::default(),
    };
    state.reset(codex);
    state
}

// TODO: Need for process management later, when save button is hit, recreate the default state for
// the creator
/// Creates the state for the task creation date entry
///
/// Polls the current date every time the function is called
pub fn make_creator_task_creation_date_entry_textbox_state(codex: &Codex) -> TextBoxState {
    let mut now = Utc::now();
    // automatic time zone detection
    now.with_auto_offset();
    let str = now.date().to_string();
    TextBoxState {
        active: false,
        cursor: Some(str.chars().count()),
        text: Text::new(str, codex).align_center(),
    }
}

pub fn make_creator_task_save_button_state() -> ButtonState {
    ButtonState::default()
}

pub fn create_creator_task_forget_button_state() -> ButtonState {
    ButtonState::default()
}

pub fn make_creator_task_prio_entry_textbox_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_center(),
    }
}

pub fn make_creator_task_context_tags_text_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_center(),
    }
}

pub fn make_creator_task_project_tags_text_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_center(),
    }
}

pub fn make_creator_task_special_tags_text_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_center(),
    }
}

pub fn make_creator_task_entry_textbox_state(codex: &Codex) -> TextBoxState {
    TextBoxState {
        active: false,
        cursor: Some(0),
        text: Text::new("", codex).align_center().align_vertically(),
    }
}
