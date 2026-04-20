use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{
        Text,
        stateful::{Button, MenuButton, TextBox},
        traits::Widget,
    },
};

use crate::startup::Environment;

pub fn render_header(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    last_frame_dur: u128,
    env: &mut Environment,
) {
    render_header_file_menu_button(canvas, codex, layout_atlas, clickable_regions, env);
    render_header_fps(canvas, codex, layout_atlas, last_frame_dur, env);
    render_header_file_path(canvas, codex, layout_atlas, env);
}

fn render_header_fps(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    last_frame_dur: u128,
    env: &Environment,
) {
    let fps = {
        if last_frame_dur == 0 {
            0
        } else {
            1000 / last_frame_dur
        }
    };
    let rect = layout_atlas.get_known_rect("header_fps");
    let default_style = env.styles.get_default();
    let mut text = Text::new(format!("FPS: {}", fps), codex)
        .align_center()
        .align_vertically()
        .with_style(default_style);
    text.render(canvas, rect, codex);
}

fn render_header_file_path(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &Environment,
) {
    let rect = layout_atlas.get_known_rect("header_file_path");
    let default_style = env.styles.get_default();
    let path = env.list.get_path();
    let mut text = Text::new(format!("Current file: {}", path.display()), codex)
        .align_center()
        .align_vertically()
        .with_style(default_style);
    text.render(canvas, rect, codex);
}

fn render_header_file_menu_button(
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

    clickable_regions.insert("header_file_menu_button".to_string(), rect);

    // Borrow checker fix: Iterate once to gather multiple mutable references from the same map.
    let mut file_button_state = None;
    let mut new_file_button_state = None;
    let mut new_file_textbox_state = None;
    let mut load_file_button_state = None;
    let mut load_file_textbox_state = None;
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
            "header_file_menu_sub_load_textbox_state" => {
                load_file_textbox_state = Some(state.as_text_box_mut().expect("Must be a textbox"))
            }
            "header_file_menu_sub_forget_button_state" => {
                forget_file_button_state = Some(state.as_button_mut().expect("Must be a button"))
            }
            _ => {
                if key.starts_with("header_file_menu_sub_forget_button_") {
                    let button = state.as_button_mut().expect("Must be a textbox");
                    forget_file_sub_button_states.push(button);
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
    let load_file_textbox_state = load_file_textbox_state.expect("Key must exist");
    let forget_file_button_state = forget_file_button_state.expect("Key must exist");
    let forget_file_button_clicked = forget_file_button_state.clicked;
    debug_assert!(forget_file_sub_button_states.len() == path_amount);

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

    let mut new_file_textbox = TextBox::new(new_file_textbox_state);
    new_file_textbox.style(editable_active);
    let mut new_menu = vec![new_file_textbox];

    let mut load_file_textbox = TextBox::new(load_file_textbox_state);
    load_file_textbox.style(editable_active);
    let mut load_menu = vec![load_file_textbox];

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
