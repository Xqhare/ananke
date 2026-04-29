use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{Focus, MenuFocus, creator::update_render_list, header::handle_key_textbox_newfile},
    keys::{
        MENU_SEARCH_PRIO_TEXTBOX, MENU_SEARCH_PRIO_TEXTBOX_STATE, MENU_SEARCH_TEXTBOX,
        MENU_SEARCH_TEXTBOX_STATE, MENU_SHOW_DROPDOWN, MENU_SHOW_DROPDOWN_ALL,
        MENU_SHOW_DROPDOWN_DONE, MENU_SHOW_DROPDOWN_OPEN, MENU_SHOW_DROPDOWN_STATE,
        MENU_SORT_DROPDOWN, MENU_SORT_DROPDOWN_COMPLETION, MENU_SORT_DROPDOWN_INCEPTION,
        MENU_SORT_DROPDOWN_NONE, MENU_SORT_DROPDOWN_PRIO, MENU_SORT_DROPDOWN_STATE,
    },
    startup::Environment,
    utils::toggle_dropdown,
};

pub fn handle_key_menu(
    key_event: &KeyEvent,
    env: &mut Environment,
    focus: &MenuFocus,
    codex: &Codex,
) -> Option<()> {
    let name = match focus {
        MenuFocus::Text => MENU_SEARCH_TEXTBOX_STATE,
        MenuFocus::Priority => MENU_SEARCH_PRIO_TEXTBOX_STATE,
    };

    handle_key_textbox_newfile(name, key_event, env, codex)
}
pub fn handle_menu_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    match name {
        MENU_SHOW_DROPDOWN => {
            toggle_dropdown(env, MENU_SHOW_DROPDOWN_STATE);
        }
        MENU_SHOW_DROPDOWN_ALL => {
            toggle_dropdown(env, MENU_SHOW_DROPDOWN_STATE);
            set_show_dropdown_selected(env, 0);
            update_render_list(env);
        }
        MENU_SHOW_DROPDOWN_DONE => {
            toggle_dropdown(env, MENU_SHOW_DROPDOWN_STATE);
            set_show_dropdown_selected(env, 1);
            update_render_list(env);
        }
        MENU_SHOW_DROPDOWN_OPEN => {
            toggle_dropdown(env, MENU_SHOW_DROPDOWN_STATE);
            set_show_dropdown_selected(env, 2);
            update_render_list(env);
        }
        MENU_SEARCH_TEXTBOX => {
            let state = env
                .states
                .get_mut(MENU_SEARCH_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            return Focus::Menu(MenuFocus::Text);
        }
        MENU_SEARCH_PRIO_TEXTBOX => {
            let state = env
                .states
                .get_mut(MENU_SEARCH_PRIO_TEXTBOX_STATE)
                .unwrap()
                .as_text_box_mut()
                .unwrap();
            state.active = true;
            state.text.set_content("", codex);
            return Focus::Menu(MenuFocus::Priority);
        }
        MENU_SORT_DROPDOWN => {
            toggle_dropdown(env, MENU_SORT_DROPDOWN_STATE);
        }
        MENU_SORT_DROPDOWN_NONE => {
            toggle_dropdown(env, MENU_SORT_DROPDOWN_STATE);
            set_sort_dropdown_selected(env, 0);
            update_render_list(env);
        }
        MENU_SORT_DROPDOWN_PRIO => {
            toggle_dropdown(env, MENU_SORT_DROPDOWN_STATE);
            set_sort_dropdown_selected(env, 1);
            update_render_list(env);
        }
        MENU_SORT_DROPDOWN_INCEPTION => {
            toggle_dropdown(env, MENU_SORT_DROPDOWN_STATE);
            set_sort_dropdown_selected(env, 2);
            update_render_list(env);
        }
        MENU_SORT_DROPDOWN_COMPLETION => {
            toggle_dropdown(env, MENU_SORT_DROPDOWN_STATE);
            set_sort_dropdown_selected(env, 3);
            update_render_list(env);
        }

        _ => {}
    }
    Focus::None
}

fn set_sort_dropdown_selected(env: &mut Environment, selected: usize) {
    let state = env
        .states
        .get_mut(MENU_SORT_DROPDOWN_STATE)
        .unwrap()
        .as_dropdown_mut()
        .unwrap();
    state.list_state.selected = Some(selected);
}

fn set_show_dropdown_selected(env: &mut Environment, selected: usize) {
    let state = env
        .states
        .get_mut(MENU_SHOW_DROPDOWN_STATE)
        .unwrap()
        .as_dropdown_mut()
        .unwrap();
    state.list_state.selected = Some(selected);
}
