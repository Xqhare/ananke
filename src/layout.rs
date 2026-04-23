use talos::{
    LayoutBuilder,
    atlases::LayoutAtlas,
    layout::{Constraint, Direction, Layout, Rect},
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
        .add_constraint(Constraint::Length(14))
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
    let layout = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .margin(1)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(*menu_rect);
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
    let sort = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Length(10))
        .add_constraint(Constraint::Min(1))
        .build()
        .split(layout[1]);
    debug_assert!(sort.len() == 2);
    vec![
        ("menu_rect".to_string(), *menu_rect),
        ("menu_sort_button_text".to_string(), buttons[0]),
        ("menu_sort_button".to_string(), buttons[1]),
        ("menu_sort_prio_button_text".to_string(), buttons[2]),
        ("menu_sort_prio_button".to_string(), buttons[3]),
        ("menu_search_text".to_string(), sort[0]),
        ("menu_search_textbox".to_string(), sort[1]),
    ]
}

fn make_creator_layout(creator_rect: &Rect) -> Vec<(String, Rect)> {
    let layout = LayoutBuilder::new()
        .direction(Direction::Vertical)
        .margin(1)
        .add_constraint(Constraint::Length(3))
        .add_constraint(Constraint::Length(3))
        .add_constraint(Constraint::Length(3))
        .add_constraint(Constraint::Length(3))
        .build()
        .split(*creator_rect);
    debug_assert!(layout.len() == 4);

    let row0 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Length(10))
        .build()
        .split(layout[0]);
    debug_assert!(row0.len() == 2);

    let row1 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(layout[1]);
    debug_assert!(row1.len() == 2);

    let row2 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(layout[2]);
    debug_assert!(row2.len() == 2);

    let row3 = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(20))
        .add_constraint(Constraint::Percentage(60))
        .add_constraint(Constraint::Percentage(20))
        .build()
        .split(layout[3]);
    debug_assert!(row3.len() == 3);

    let row3_middle_buttons = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Percentage(50))
        .add_constraint(Constraint::Percentage(50))
        .build()
        .split(row3[1]);
    debug_assert!(row3_middle_buttons.len() == 2);

    let help_page = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .margin(1)
        .add_constraint(Constraint::Percentage(70))
        .add_constraint(Constraint::Percentage(30))
        .build()
        .split(*creator_rect);
    debug_assert!(help_page.len() == 2);

    vec![
        ("creator_rect".to_string(), *creator_rect),
        ("creator_textbox_task".to_string(), row0[0]),
        ("creator_button_prio".to_string(), row0[1]),
        ("creator_textbox_inception".to_string(), row1[0]),
        ("creator_textbox_context_tags".to_string(), row1[1]),
        ("creator_textbox_project_tags".to_string(), row2[0]),
        ("creator_textbox_special_tags".to_string(), row2[1]),
        (
            "creator_button_forget_new_task".to_string(),
            row3_middle_buttons[0],
        ),
        (
            "creator_button_add_new_task".to_string(),
            row3_middle_buttons[1],
        ),
        ("creator_help_page_left".to_string(), help_page[0]),
        ("creator_help_page_right".to_string(), help_page[1]),
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
        ("header_file_button".to_string(), buttons[0]),
        ("header_save_button".to_string(), buttons[1]),
        ("header_help_button".to_string(), buttons[2]),
        ("header_exit_button".to_string(), buttons[3]),
        ("header_fps".to_string(), stats[1]),
        ("header_file_path".to_string(), stats[2]),
    ]
}
