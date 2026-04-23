use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    input::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
};

mod header;

use crate::{
    input::header::{handle_header_mouse, handle_key_textbox_newfile},
    startup::Environment,
    utils::{add_load_n_forget_button_states, ensure_focus_on_active_textfield},
};

/// The focus of the program
///
/// Used for keyboard input capture and ensuring that the focus is on the active textfield
#[derive(Clone, Copy, Debug)]
pub enum Focus {
    /// No focus
    None,
    /// Focus on the header new-file textbox
    HeaderFileNewTextBox,
}

pub fn process_input(
    codex: &Codex,
    events: Option<&[Event]>,
    env: &mut Environment,
    clickable_regions: &BTreeMap<String, Rect>,
    focus: &Focus,
) -> Option<Focus> {
    if let Some(events) = events {
        for event in events.iter() {
            match event {
                Event::KeyEvent(key_event) => {
                    // In any case, if the escape key is pressed, exit all special modes
                    if key_event.code == KeyCode::Esc {
                        return Some(Focus::None);
                    }
                    match focus {
                        Focus::None => {
                            handle_key_normal(key_event, env);
                            return None;
                        }
                        Focus::HeaderFileNewTextBox => {
                            if let Some(_) = handle_key_textbox_newfile(
                                "header_file_menu_sub_new_textbox_state",
                                key_event,
                                env,
                                codex,
                            ) {
                                // Add the load/forget buttons
                                add_load_n_forget_button_states(env);
                                // Lastly close the menu
                                let state = env
                                    .states
                                    .get_mut("header_file_menu_sub_new_button_state")
                                    .unwrap()
                                    .as_button_mut()
                                    .unwrap();
                                state.clicked = false;
                            };
                            return None;
                        }
                    }
                }
                Event::MouseEvent(mouse_event) => {
                    return Some(handle_mouse(mouse_event, env, clickable_regions));
                }
                _ => {
                    return None;
                }
            }
        }
    }
    ensure_focus_on_active_textfield(env, focus);
    None
}

/// Handles the key events for the normal mode
fn handle_key_normal(key_event: &KeyEvent, env: &mut Environment) {
    match key_event.code {
        KeyCode::Char(c) => match c {
            'q' | 'Q' => {
                env.run = false;
            }
            _ => {}
        },
        _ => {}
    }
}

/// Handles the mouse events
fn handle_mouse(
    mouse_event: &MouseEvent,
    env: &mut Environment,
    clickable_regions: &BTreeMap<String, Rect>,
) -> Focus {
    for (name, rect) in clickable_regions.iter() {
        if rect.contains(mouse_event.column, mouse_event.row) {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    if name.contains("header") {
                        return handle_header_mouse(env, name);
                    }
                }
                _ => {}
            }
        }
    }
    Focus::None
}
