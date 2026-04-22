use std::collections::BTreeMap;

use anansi::List;
use talos::{
    Talos,
    atlases::LayoutAtlas,
    input::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
    widgets::stateful::States,
};

use crate::startup::Environment;

pub fn process_input(
    events: Option<&[Event]>,
    env: &mut Environment,
    clickable_regions: &BTreeMap<String, Rect>,
) {
    if let Some(events) = events {
        for event in events.iter() {
            match event {
                Event::KeyEvent(key_event) => {
                    handle_key_normal(key_event, env);
                }
                Event::MouseEvent(mouse_event) => {
                    handle_mouse(mouse_event, env, clickable_regions);
                }
                _ => {}
            }
        }
    }
}

fn handle_key_normal(key_event: &KeyEvent, env: &mut Environment) {
    match key_event.code {
        KeyCode::Char(c) => match c {
            'q' => {
                env.run = false;
            }
            _ => {}
        },
        _ => {}
    }
}

fn handle_mouse(
    mouse_event: &MouseEvent,
    env: &mut Environment,
    clickable_regions: &BTreeMap<String, Rect>,
) {
    for (name, rect) in clickable_regions.iter() {
        if rect.contains(mouse_event.column, mouse_event.row) {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    if name.contains("header") {
                        handle_header_mouse(env, name);
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_header_mouse(env: &mut Environment, name: &str) {
    match name {
        "header_file_menu_button" => {
            let state = env
                .states
                .get_mut("header_file_menu_button_main_button_state")
                .unwrap()
                .as_button_mut()
                .unwrap();
            if state.clicked {
                state.clicked = false;
            } else {
                state.clicked = true;
            }
        }
        "header_file_menu_sub_new_textbox" => {
            let state = env
                .states
                .get_mut("header_file_menu_sub_new_button_state")
                .unwrap()
                .as_button_mut()
                .unwrap();
            if state.clicked {
                state.clicked = false;
            } else {
                state.clicked = true;
            }
        }
        "header_file_menu_sub_load_button" => {
            let state = env
                .states
                .get_mut("header_file_menu_sub_load_button_state")
                .unwrap()
                .as_button_mut()
                .unwrap();
            if state.clicked {
                state.clicked = false;
            } else {
                state.clicked = true;
            }
        }
        "header_file_menu_sub_forget_button" => {
            let state = env
                .states
                .get_mut("header_file_menu_sub_forget_button_state")
                .unwrap()
                .as_button_mut()
                .unwrap();
            if state.clicked {
                state.clicked = false;
            } else {
                state.clicked = true;
            }
        }
        _ => {
            if name.contains("header_file_menu_sub_forget_button_") {
                let button = env
                    .states
                    .get_mut("header_file_menu_sub_forget_button_state")
                    .unwrap()
                    .as_button_mut()
                    .unwrap();
                button.clicked = false;
                let index = name.split("_").last().unwrap().parse::<u32>().unwrap();
                let mut path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let paths_ary = path.as_object_mut().unwrap().get_mut("paths").unwrap();
                let path = paths_ary.as_array().unwrap().get(index as usize).unwrap();
                let path = path.as_string().unwrap();
                if !path.contains("/Ananke/default-list.txt") {
                    let paths_ary = paths_ary.as_array_mut().unwrap();
                    paths_ary.remove(index as usize);
                }
            } else if name.contains("header_file_menu_sub_load_button_") {
                let button = env
                    .states
                    .get_mut("header_file_menu_sub_load_button_state")
                    .unwrap()
                    .as_button_mut()
                    .unwrap();
                button.clicked = false;
                // TODO: Consider this one unwrap
                let _ = env.list.save().unwrap();
                let index = name.split("_").last().unwrap().parse::<u32>().unwrap();
                let path = env.disk_env.brigid.get_file("config.xff").unwrap();
                let path = path.as_object().unwrap().get("paths").unwrap();
                let path = path.as_array().unwrap().get(index as usize).unwrap();
                let path = path.as_string().unwrap();
                if env.list.get_path() != path {
                    env.list = List::new(path);
                }
            }
        }
    }
}
