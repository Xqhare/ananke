use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::{Canvas, Colour, Normal},
    widgets::{
        Block,
        stateful::{BlockBox, Button, MenuButton, TextBox},
        traits::Widget,
    },
};

use crate::startup::Environment;

pub fn render_header_file_menu_button(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let path_amount = env.path_amount;
    let all_paths = {
        let mut paths = Vec::with_capacity(path_amount);
        let config = env.disk_env.brigid.get_file("config.xff").unwrap();
        let ary = config
            .as_object()
            .unwrap()
            .get("paths")
            .unwrap()
            .as_array()
            .unwrap();
        for path in ary {
            if let Some(path) = path.as_string() {
                paths.push(path.clone());
            } else {
                // Should be unreachable
                debug_assert!(false);
            }
        }
        paths
    };

    let rect = layout_atlas.get_known_rect("header_file_button");
    let default_style = env.styles.get_default();
    let default_clicked_style = env.styles.get_known_style("default_inverted");
    let editable_active = env.styles.get_known_style("editable_active");

    // Borrow checker fix: Iterate once to gather multiple mutable references from the same map.
    let mut file_button_state = None;
    let mut new_file_button_state = None;
    let mut new_file_textbox_state = None;
    let mut load_file_button_state = None;
    let mut load_file_sub_button_states = Vec::with_capacity(path_amount);
    let mut forget_file_button_state = None;
    let mut forget_file_sub_button_states = Vec::with_capacity(path_amount);

    for (key, state) in env.states.iter_mut() {
        match key.as_str() {
            "header_file_menu_button_main_button_state" => {
                file_button_state = Some(state.as_button_mut().expect("Must be a button"))
            }
            "header_file_menu_sub_new_button_state" => {
                new_file_button_state = Some(state.as_button_mut().expect("Must be a button"))
            }
            "header_file_menu_sub_new_textbox_state" => {
                new_file_textbox_state = Some(state.as_text_box_mut().expect("Must be a textbox"))
            }
            "header_file_menu_sub_load_button_state" => {
                load_file_button_state = Some(state.as_button_mut().expect("Must be a button"))
            }
            "header_file_menu_sub_forget_button_state" => {
                forget_file_button_state = Some(state.as_button_mut().expect("Must be a button"))
            }
            _ => {
                // Keys are sorted lexicographically -> 0 will always be first, 1 second, etc
                if key.starts_with("header_file_menu_sub_forget_button_") {
                    let button = state.as_button_mut().expect("Must be a button");
                    forget_file_sub_button_states.push(button);
                } else if key.starts_with("header_file_menu_sub_load_button_") {
                    let button = state.as_button_mut().expect("Must be a button");
                    load_file_sub_button_states.push(button);
                }
            }
        }
    }

    let file_button_state = file_button_state.expect("Key must exist");
    let file_button_clicked = file_button_state.clicked;
    let new_file_button_state = new_file_button_state.expect("Key must exist");
    let new_file_button_clicked = new_file_button_state.clicked;
    let new_file_textbox_state = new_file_textbox_state.expect("Key must exist");
    let load_file_button_state = load_file_button_state.expect("Key must exist");
    let load_file_button_clicked = load_file_button_state.clicked;
    debug_assert!(load_file_sub_button_states.len() == path_amount);
    let forget_file_button_state = forget_file_button_state.expect("Key must exist");
    let forget_file_button_clicked = forget_file_button_state.clicked;
    debug_assert!(forget_file_sub_button_states.len() == path_amount);

    update_clickable_regions(
        clickable_regions,
        rect,
        file_button_clicked,
        new_file_button_clicked,
        load_file_button_clicked,
        forget_file_button_clicked,
        path_amount,
    );

    let mut file_button =
        Button::new("File", file_button_state, codex).with_clicked_style(default_clicked_style);
    file_button.style(default_style);

    let mut new_file_button =
        Button::new("New", new_file_button_state, codex).with_clicked_style(default_clicked_style);
    new_file_button.style(default_style);

    let mut load_file_button = Button::new("Load", load_file_button_state, codex)
        .with_clicked_style(default_clicked_style);
    load_file_button.style(default_style);

    let mut forget_file_button = Button::new("Forget", forget_file_button_state, codex)
        .with_clicked_style(default_clicked_style);
    forget_file_button.style(default_style);

    let cursor_style = env.styles.get_known_style("cursor");
    let mut new_file_textbox =
        TextBox::new(new_file_textbox_state).with_highlight_style(cursor_style);
    new_file_textbox.style(editable_active);
    let mut block = Block::new()
        .with_bg_fill()
        .with_style(editable_active)
        .with_fat_border();
    let block_box = BlockBox::new(&mut block, &mut new_file_textbox);
    let mut new_menu = vec![block_box];

    let mut load_menu = Vec::with_capacity(path_amount);
    for (index, sub_button_state) in load_file_sub_button_states.iter_mut().enumerate() {
        let mut button = Button::new(all_paths[index].clone(), sub_button_state, codex)
            .with_clicked_style(default_clicked_style);
        button.style(default_style);
        load_menu.push(button);
    }

    let mut forget_menu = Vec::with_capacity(path_amount);
    for (index, sub_button_state) in forget_file_sub_button_states.iter_mut().enumerate() {
        let mut button = Button::new(all_paths[index].clone(), sub_button_state, codex)
            .with_clicked_style(default_clicked_style);
        button.style(default_style);
        forget_menu.push(button);
    }

    let mut new_file_menu_button =
        MenuButton::new(new_file_button, new_menu.iter_mut()).with_horizontal_layout();
    if new_file_button_clicked {
        new_file_menu_button.style(default_clicked_style);
    } else {
        new_file_menu_button.style(default_style);
    }

    let mut load_file_menu_button =
        MenuButton::new(load_file_button, load_menu.iter_mut()).with_horizontal_layout();
    if load_file_button_clicked {
        load_file_menu_button.style(default_clicked_style);
    } else {
        load_file_menu_button.style(default_style);
    }

    let mut forget_file_menu_button =
        MenuButton::new(forget_file_button, forget_menu.iter_mut()).with_horizontal_layout();
    if forget_file_button_clicked {
        forget_file_menu_button.style(default_clicked_style);
    } else {
        forget_file_menu_button.style(default_style);
    }

    let mut file_menu = vec![
        new_file_menu_button,
        load_file_menu_button,
        forget_file_menu_button,
    ];
    let mut file_menu_button =
        MenuButton::new(file_button, file_menu.iter_mut()).with_vertical_layout();
    if file_button_clicked {
        file_menu_button.style(default_clicked_style);
    } else {
        file_menu_button.style(default_style);
    }

    file_menu_button.render(canvas, rect, codex);
}

fn update_clickable_regions(
    clickable_regions: &mut BTreeMap<String, Rect>,
    rect: Rect,
    file_button_clicked: bool,
    new_file_button_clicked: bool,
    load_file_button_clicked: bool,
    forget_file_button_clicked: bool,
    path_amount: usize,
) {
    // Names are prepended with "a0_" to ensure they are first in the BTreeMap
    //
    // Some buttons are drawn over other clickable regions, this way ensures they are looped over
    // (and 'clicked on') first inside the input processing
    clickable_regions.insert("a0_header_file_menu_button".to_string(), rect);

    if file_button_clicked {
        clickable_regions.insert(
            "a0_header_file_menu_sub_new_button".to_string(),
            Rect::new(
                rect.x,
                rect.y.saturating_add(3 * 1),
                rect.width,
                rect.height,
            ),
        );
        if new_file_button_clicked {
            clickable_regions.insert(
                "a0_header_file_menu_sub_new_textbox".to_string(),
                Rect::new(
                    rect.x.saturating_add(rect.width),
                    rect.y.saturating_add(3 * 1),
                    rect.width,
                    rect.height,
                ),
            );
        }
        clickable_regions.insert(
            "a0_header_file_menu_sub_load_button".to_string(),
            Rect::new(
                rect.x,
                rect.y.saturating_add(3 * 2),
                rect.width,
                rect.height,
            ),
        );
        if load_file_button_clicked {
            for n in 0..path_amount {
                clickable_regions.insert(
                    format!("a0_header_file_menu_sub_load_button_{}", n),
                    Rect::new(
                        rect.x.saturating_add(rect.width * (n as u16 + 1)),
                        rect.y.saturating_add(3 * 2),
                        rect.width,
                        rect.height,
                    ),
                );
            }
        }
        clickable_regions.insert(
            "a0_header_file_menu_sub_forget_button".to_string(),
            Rect::new(
                rect.x,
                rect.y.saturating_add(3 * 3),
                rect.width,
                rect.height,
            ),
        );
        if forget_file_button_clicked {
            for n in 0..path_amount {
                clickable_regions.insert(
                    format!("a0_header_file_menu_sub_forget_button_{}", n),
                    Rect::new(
                        rect.x.saturating_add(rect.width * (n as u16 + 1)),
                        rect.y.saturating_add(3 * 3),
                        rect.width,
                        rect.height,
                    ),
                );
            }
        }
    }
}
