use talos::{
    LayoutBuilder,
    atlases::LayoutAtlas,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::keys::{
    CREATOR_CLEAR_BUTTON, CREATOR_HELP_PAGE_LEFT, CREATOR_HELP_PAGE_RIGHT,
    CREATOR_INCEPTION_ENTRY_TEXTBOX, CREATOR_INCEPTION_TEXT, CREATOR_PRIO_ENTRY_TEXTBOX,
    CREATOR_PRIO_TEXT, CREATOR_RECT, CREATOR_SAVE_BUTTON, CREATOR_TASK_ENTRY_TEXTBOX,
    CREATOR_TEXT_CONTEXT_TAGS, CREATOR_TEXT_PROJECT_TAGS, CREATOR_TEXT_SPECIAL_TAGS,
    HEADER_EXIT_BUTTON, HEADER_FILE_MENU_BUTTON, HEADER_FILE_PATH, HEADER_FPS, HEADER_HELP_BUTTON,
    HEADER_SAVE_BUTTON, MENU_RECT, MENU_SEARCH_PRIO_TEXT, MENU_SEARCH_PRIO_TEXTBOX,
    MENU_SEARCH_TEXTBOX, MENU_SHOW_DROPDOWN, MENU_SHOW_DROPDOWN_TEXT, MENU_SORT_DROPDOWN,
    MENU_SORT_DROPDOWN_TEXT,
};

/// Builds the basic layout of Ananke.
///
/// It splits the screen into the 4 needed sectors vertically.
///
/// 1. Header - Buttons, set size of 3
/// 2. Creator - Task creation, set size of 14
/// 3. Menu - Sorting etc, set size of 5
/// 4. Task list - Task to be rendered, rest of the screen
pub fn make_layout() -> Layout {
    LayoutBuilder::new()
        .direction(Direction::Vertical)
        .add_constraint(Constraint::Length(3))
        .add_constraint(Constraint::Length(16))
        .add_constraint(Constraint::Length(5))
        .add_constraint(Constraint::Min(1))
        .build()
}

pub fn make_frame_layout(screen_rect: &Rect, layout: &Layout) -> LayoutAtlas {
    let basic_layout = layout.split(*screen_rect);
    debug_assert!(basic_layout.len() == 4);
    let mut out = LayoutAtlas::new();
    out.store.extend(make_header_layout(&basic_layout[0]));
    out.store.extend(make_creator_layout(&basic_layout[1]));
    out.store.extend(make_menu_layout(&basic_layout[2]));
    out.store.insert("task_list".to_string(), basic_layout[3]);
    out
}

fn make_menu_layout(menu_rect: &Rect) -> Vec<(String, Rect)> {
    let mut layout = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .margin(1)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(*menu_rect);
    debug_assert!(layout.len() == 2);
    layout[0].width -= 2;
    // You can think of this as a true flex box. The available space is split into 6 equal parts (or as close as possible)
    let mut buttons = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .build()
        .split(layout[0]);
    debug_assert!(buttons.len() == 6);
    buttons[2].width -= 1;
    buttons[3].x -= 1;
    buttons[4].x -= 1;
    vec![
        (MENU_RECT.to_string(), *menu_rect),
        (MENU_SHOW_DROPDOWN_TEXT.to_string(), buttons[0]),
        (MENU_SHOW_DROPDOWN.to_string(), buttons[1]),
        (MENU_SORT_DROPDOWN_TEXT.to_string(), buttons[2]),
        (MENU_SORT_DROPDOWN.to_string(), buttons[3]),
        (MENU_SEARCH_PRIO_TEXT.to_string(), buttons[4]),
        (MENU_SEARCH_PRIO_TEXTBOX.to_string(), buttons[5]),
        (MENU_SEARCH_TEXTBOX.to_string(), layout[1]),
    ]
}

fn make_creator_layout(creator_rect: &Rect) -> Vec<(String, Rect)> {
    let layout = LayoutBuilder::new()
        .direction(Direction::Vertical)
        .margin(1)
        .add_constraint(Constraint::Length(5))
        .add_constraint(Constraint::Length(3))
        .add_constraint(Constraint::Length(3))
        .add_constraint(Constraint::Length(3))
        .build()
        .split(*creator_rect);
    debug_assert!(layout.len() == 4);

    let row1 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(layout[1]);
    debug_assert!(row1.len() == 2);

    let mut row1_sub = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(25))
        .add_constraint(Constraint::Percentage(75))
        .build()
        .split(row1[0]);
    debug_assert!(row1_sub.len() == 2);
    row1_sub[0].width -= 1;
    row1_sub[1].x -= 1;
    row1_sub[1].width += 1;

    let row_1_prio = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(35))
        .add_constraint(Constraint::Percentage(65))
        .build()
        .split(row1_sub[0]);
    debug_assert!(row_1_prio.len() == 2);

    let mut row_1_inception = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(30))
        .add_constraint(Constraint::Percentage(70))
        .build()
        .split(row1_sub[1]);
    debug_assert!(row_1_inception.len() == 2);
    row_1_inception[1].x += 3;
    row_1_inception[1].width -= 3;

    let mut row2 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(layout[2]);
    debug_assert!(row2.len() == 2);
    row2[0].width -= 2;

    let row3 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(25))
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(25))
        .build()
        .split(layout[3]);
    debug_assert!(row3.len() == 3);

    let mut row3_middle_buttons = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(row3[1]);
    debug_assert!(row3_middle_buttons.len() == 2);
    row3_middle_buttons[0].x -= 1;
    row3_middle_buttons[1].x += 1;

    let help_page = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .margin(1)
        .add_constraint(Constraint::Percentage(70))
        .add_constraint(Constraint::Percentage(30))
        .build()
        .split(*creator_rect);
    debug_assert!(help_page.len() == 2);

    vec![
        (CREATOR_RECT.to_string(), *creator_rect),
        (CREATOR_TASK_ENTRY_TEXTBOX.to_string(), layout[0]),
        (CREATOR_PRIO_TEXT.to_string(), row_1_prio[0]),
        (CREATOR_PRIO_ENTRY_TEXTBOX.to_string(), row_1_prio[1]),
        (CREATOR_INCEPTION_TEXT.to_string(), row_1_inception[0]),
        (
            CREATOR_INCEPTION_ENTRY_TEXTBOX.to_string(),
            row_1_inception[1],
        ),
        (CREATOR_TEXT_CONTEXT_TAGS.to_string(), row1[1]),
        (CREATOR_TEXT_PROJECT_TAGS.to_string(), row2[0]),
        (CREATOR_TEXT_SPECIAL_TAGS.to_string(), row2[1]),
        (CREATOR_CLEAR_BUTTON.to_string(), row3_middle_buttons[0]),
        (CREATOR_SAVE_BUTTON.to_string(), row3_middle_buttons[1]),
        (CREATOR_HELP_PAGE_LEFT.to_string(), help_page[0]),
        (CREATOR_HELP_PAGE_RIGHT.to_string(), help_page[1]),
    ]
}

fn make_header_layout(header_rect: &Rect) -> Vec<(String, Rect)> {
    let layout = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(*header_rect);
    debug_assert!(layout.len() == 2);
    let buttons = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(25))
        .add_constraint(Constraint::Percentage(25))
        .add_constraint(Constraint::Percentage(25))
        .add_constraint(Constraint::Percentage(25))
        .build()
        .split(layout[0]);
    debug_assert!(buttons.len() == 4);
    let stats = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(10))
        .add_constraint(Constraint::Percentage(20))
        .add_constraint(Constraint::Percentage(70))
        .build()
        .split(layout[1]);
    debug_assert!(stats.len() == 3);
    vec![
        (HEADER_FILE_MENU_BUTTON.to_string(), buttons[0]),
        (HEADER_SAVE_BUTTON.to_string(), buttons[1]),
        (HEADER_HELP_BUTTON.to_string(), buttons[2]),
        (HEADER_EXIT_BUTTON.to_string(), buttons[3]),
        (HEADER_FPS.to_string(), stats[1]),
        (HEADER_FILE_PATH.to_string(), stats[2]),
    ]
}
