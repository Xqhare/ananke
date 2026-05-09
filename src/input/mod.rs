use std::collections::BTreeMap;

use talos::{
    codex::Codex,
    input::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::Rect,
    widgets::stateful::TextBoxState,
};

mod creator;
mod header;
mod menu;

use crate::{
    input::{
        creator::{handle_key_creator, mouse::handle_creator_mouse},
        header::{handle_header_mouse, handle_header_newfile_input},
        menu::{handle_key_menu, handle_menu_mouse},
    },
    startup::Environment,
    state::Focus,
    utils::{add_load_n_forget_button_states, ensure_focus_on_active_textfield},
};

/// Handles the generic textbox input (backspace, char entry)
///
/// Returns true if the content changed
pub fn handle_generic_textbox_input(
    key_event: &KeyEvent,
    state: &mut TextBoxState,
    codex: &Codex,
) -> bool {
    let mut content = state.text.get_content().to_string();
    let changed;

    match key_event.code {
        KeyCode::Backspace => {
            content.pop();
            changed = true;
        }
        KeyCode::Char(c) => {
            content.push(c);
            changed = true;
        }
        _ => {
            changed = false;
        }
    }

    if changed {
        state.text.set_content(&content, codex);
        state.cursor = Some(state.text.len());
    }

    changed
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
                            let state = env.ui_state.active_textbox_mut(focus)?;
                            handle_generic_textbox_input(key_event, state, codex);
                            if let Some(_) = handle_header_newfile_input(key_event, env, codex) {
                                // Add the load/forget buttons
                                add_load_n_forget_button_states(env);
                                // Lastly close the menu
                                let state = &mut env.ui_state.header.file_menu_sub_new_button;
                                state.clicked = false;
                            };
                            return None;
                        }
                        Focus::Creator(any) => {
                            // Handle the creator. Returns `Some(())` if enter was hit.
                            // We do not save then, but we loose the focus
                            //
                            // The todo.txt format only allows for one line per task.
                            if handle_key_creator(key_event, env, &any, codex).is_some() {
                                return Some(Focus::None);
                            }
                        }
                        Focus::Menu(any) => {
                            if handle_key_menu(key_event, env, &any, codex).is_some() {
                                return Some(Focus::None);
                            }
                        }
                    }
                }
                Event::MouseEvent(mouse_event) => {
                    return Some(handle_mouse(
                        mouse_event,
                        env,
                        clickable_regions,
                        codex,
                        focus,
                    ));
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
    codex: &Codex,
    current_focus: &Focus,
) -> Focus {
    // Doesn't matter where the mouse is when scrolling, always scroll the list
    if mouse_event.kind == MouseEventKind::ScrollUp
        || mouse_event.kind == MouseEventKind::ScrollDown
    {
        return handle_scrolling(mouse_event, env, current_focus);
    }
    for (name, rect) in clickable_regions.iter() {
        if rect.contains(mouse_event.column, mouse_event.row) {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    if name.contains("header") {
                        return handle_header_mouse(env, name);
                    } else if name.contains("creator") {
                        return handle_creator_mouse(env, name, codex);
                    } else if name.contains("menu") {
                        return handle_menu_mouse(env, name, codex);
                    }
                }
                _ => {}
            }
        }
    }
    Focus::None
}

fn handle_scrolling(
    mouse_event: &MouseEvent,
    env: &mut Environment,
    current_focus: &Focus,
) -> Focus {
    // Ignore the `x_offset` as there is only vertical scrolling
    if mouse_event.kind == MouseEventKind::ScrollUp {
        let list_state = &mut env.ui_state.task_table;
        let current_offset = list_state.y_offset;
        if current_offset > 0 {
            list_state.y_offset = current_offset - 1;
        }
    } else if mouse_event.kind == MouseEventKind::ScrollDown {
        let list_max = env.list.task_amount();
        let list_state = &mut env.ui_state.task_table;
        let current_offset = list_state.y_offset;
        if current_offset < list_max - 1 {
            list_state.y_offset = current_offset + 1;
        }
    }
    // Don't change the focus; Don't throw a user out of a text entry just because they scrolled - horrible UX otherwise imho
    return *current_focus;
}
