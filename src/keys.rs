pub const CREATOR_RECT: &str = "creator_rect";
pub const CREATOR_TASK_ENTRY_TEXTBOX: &str = "creator_task_textbox";
pub const CREATOR_TASK_ENTRY_TEXTBOX_STATE: &str = "creator_task_entry_textbox_state";
pub const CREATOR_PRIO_TEXT: &str = "creator_prio_text";
pub const CREATOR_PRIO_ENTRY_TEXTBOX: &str = "creator_prio_textbox";
pub const CREATOR_PRIO_ENTRY_TEXTBOX_STATE: &str = "creator_task_prio_entry_textbox_state";
pub const CREATOR_INCEPTION_TEXT: &str = "creator_inception_text";
pub const CREATOR_INCEPTION_ENTRY_TEXTBOX: &str = "creator_inception_textbox";
pub const CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE: &str =
    "creator_task_creation_date_entry_textbox_state";
pub const CREATOR_TEXT_CONTEXT_TAGS: &str = "creator_text_context_tags";
pub const CREATOR_TEXT_PROJECT_TAGS: &str = "creator_text_project_tags";
pub const CREATOR_TEXT_SPECIAL_TAGS: &str = "creator_text_special_tags";
pub const CREATOR_CLEAR_BUTTON: &str = "creator_clear_button";
pub const CREATOR_CLEAR_BUTTON_STATE: &str = "creator_clear_button_state";
pub const CREATOR_SAVE_BUTTON: &str = "creator_save_button";
pub const CREATOR_SAVE_BUTTON_STATE: &str = "creator_save_button_state";
pub const CREATOR_HELP_PAGE_LEFT: &str = "creator_help_page_left";
pub const CREATOR_HELP_PAGE_RIGHT: &str = "creator_help_page_right";

// Names are prepended with "a0_" to ensure they are first in the clickable regions BTreeMap.
// This is only relevant for the children of the file menu button, as they are drawn over parts of the
// creator interactive regions (Task text textbox & Prio textbox)
//
// Names ending with a "_" are used as base names for needed children; they add their id or index to the end
pub const HEADER_FILE_MENU_SUB_NEW_BUTTON: &str = "a0_header_file_menu_sub_new_button";
pub const HEADER_FILE_MENU_SUB_NEW_BUTTON_STATE: &str = "header_file_menu_sub_new_button_state";
pub const HEADER_FILE_MENU_SUB_NEW_TEXTBOX: &str = "a0_header_file_menu_sub_new_textbox";
pub const HEADER_FILE_MENU_SUB_NEW_TEXTBOX_STATE: &str = "header_file_menu_sub_new_textbox_state";
pub const HEADER_FILE_MENU_SUB_LOAD_BUTTON: &str = "a0_header_file_menu_sub_load_button";
pub const HEADER_FILE_MENU_SUB_LOAD_BUTTON_BASE: &str = "a0_header_file_menu_sub_load_button_";
pub const HEADER_FILE_MENU_SUB_LOAD_BUTTON_STATE: &str = "header_file_menu_sub_load_button_state";
pub const HEADER_FILE_MENU_SUB_FORGET_BUTTON: &str = "a0_header_file_menu_sub_forget_button";
pub const HEADER_FILE_MENU_SUB_FORGET_BUTTON_BASE: &str = "a0_header_file_menu_sub_forget_button_";
pub const HEADER_FILE_MENU_SUB_FORGET_BUTTON_STATE: &str =
    "header_file_menu_sub_forget_button_state";
pub const HEADER_FILE_MENU_BUTTON: &str = "header_file_menu_button";
pub const HEADER_FILE_MENU_BUTTON_STATE: &str = "header_file_menu_button_state";
pub const HEADER_SAVE_BUTTON: &str = "header_save_button";
pub const HEADER_SAVE_BUTTON_STATE: &str = "header_save_button_state";
pub const HEADER_HELP_BUTTON: &str = "header_help_button";
pub const HEADER_HELP_BUTTON_STATE: &str = "header_help_button_state";
pub const HEADER_EXIT_BUTTON: &str = "header_exit_button";
pub const HEADER_EXIT_BUTTON_STATE: &str = "header_exit_button_state";
pub const HEADER_FPS: &str = "header_fps";
pub const HEADER_FILE_PATH: &str = "header_file_path";

// _ at the end to separate from the id
pub const LIST_SINGLE_TASK_DONE_BUTTON_STATE: &str = "list_single_task_id_button_state_";
pub const LIST_SINGLE_TASK_DELETE_BUTTON_STATE: &str = "list_single_task_delete_button_state_";
pub const LIST_SINGLE_TASK_PRIO_TEXTBOX_STATE: &str = "list_single_task_prio_textbox_state_";
pub const LIST_SINGLE_TASK_TEXT_TEXTBOX_STATE: &str = "list_single_task_text_textbox_state_";

pub const MENU_RECT: &str = "menu_rect";
pub const MENU_SHOW_DROPDOWN_TEXT: &str = "menu_show_button_text";
pub const MENU_SHOW_DROPDOWN_STATE: &str = "menu_show_button_state";
pub const MENU_SHOW_DROPDOWN: &str = "menu_show_button";
pub const MENU_SORT_BUTTON_TEXT: &str = "menu_sort_button_text";
pub const MENU_SORT_BUTTON_STATE: &str = "menu_sort_button_state";
pub const MENU_SORT_BUTTON: &str = "menu_sort_button";
pub const MENU_SORT_PRIO_BUTTON_TEXT: &str = "menu_sort_prio_button_text";
pub const MENU_SORT_PRIO_TEXTBOX: &str = "menu_sort_prio_textbox";
pub const MENU_SORT_PRIO_TEXTBOX_STATE: &str = "menu_sort_prio_textbox_state";
pub const MENU_SEARCH_TEXT: &str = "menu_search_text";
pub const MENU_SEARCH_TEXTBOX: &str = "menu_search_textbox";
pub const MENU_SEARCH_TEXTBOX_STATE: &str = "menu_search_textbox_state";

pub mod styles {
    pub const CURSOR: &str = "cursor";
    pub const DEFAULT_INVERTED: &str = "default_inverted";
    pub const EDITABLE_ACTIVE: &str = "editable_active";
    pub const EDITABLE_INACTIVE: &str = "editable_inactive";
}
