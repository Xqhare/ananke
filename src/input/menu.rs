use talos::{codex::Codex, input::KeyEvent};

use crate::{
    input::{
        creator::update_render_list, handle_generic_textbox_input, header::keep_textbox_at_one_char,
    },
    keys::{
        MENU_SEARCH_PRIO_TEXTBOX, MENU_SEARCH_TEXTBOX, MENU_SHOW_DROPDOWN, MENU_SHOW_DROPDOWN_ALL,
        MENU_SHOW_DROPDOWN_DONE, MENU_SHOW_DROPDOWN_OPEN, MENU_SORT_DROPDOWN,
        MENU_SORT_DROPDOWN_COMPLETION, MENU_SORT_DROPDOWN_INCEPTION, MENU_SORT_DROPDOWN_NONE,
        MENU_SORT_DROPDOWN_PRIO,
    },
    startup::Environment,
    state::{Focus, MenuFocus},
};

pub fn handle_key_menu(
    key_event: &KeyEvent,
    env: &mut Environment,
    focus: &MenuFocus,
    codex: &Codex,
) -> Option<()> {
    if handle_generic_textbox_input(
        key_event,
        env.ui_state.active_textbox_mut(&Focus::Menu(*focus))?,
        codex,
    ) {
        if let MenuFocus::Priority = focus {
            if keep_textbox_at_one_char(env, &Focus::Menu(*focus), codex) {
                env.ui_state.menu.sort_prio_textbox.active = false;
            }
        }
        update_render_list(env);
    }
    None
}

pub fn handle_menu_mouse(env: &mut Environment, name: &str, codex: &Codex) -> Focus {
    match name {
        MENU_SHOW_DROPDOWN => {
            env.ui_state.menu.show_dropdown.expanded = !env.ui_state.menu.show_dropdown.expanded;
        }
        MENU_SHOW_DROPDOWN_ALL => {
            env.ui_state.menu.show_dropdown.expanded = !env.ui_state.menu.show_dropdown.expanded;
            env.ui_state.menu.show_dropdown.list_state.selected = Some(0);
            update_render_list(env);
        }
        MENU_SHOW_DROPDOWN_DONE => {
            env.ui_state.menu.show_dropdown.expanded = !env.ui_state.menu.show_dropdown.expanded;
            env.ui_state.menu.show_dropdown.list_state.selected = Some(1);
            update_render_list(env);
        }
        MENU_SHOW_DROPDOWN_OPEN => {
            env.ui_state.menu.show_dropdown.expanded = !env.ui_state.menu.show_dropdown.expanded;
            env.ui_state.menu.show_dropdown.list_state.selected = Some(2);
            update_render_list(env);
        }
        MENU_SEARCH_TEXTBOX => {
            env.ui_state.menu.search_textbox.active = true;
            return Focus::Menu(MenuFocus::Text);
        }
        MENU_SEARCH_PRIO_TEXTBOX => {
            env.ui_state.menu.sort_prio_textbox.active = true;
            env.ui_state
                .menu
                .sort_prio_textbox
                .text
                .set_content("", codex);
            return Focus::Menu(MenuFocus::Priority);
        }
        MENU_SORT_DROPDOWN => {
            env.ui_state.menu.sort_dropdown.expanded = !env.ui_state.menu.sort_dropdown.expanded;
        }
        MENU_SORT_DROPDOWN_NONE => {
            env.ui_state.menu.sort_dropdown.expanded = !env.ui_state.menu.sort_dropdown.expanded;
            env.ui_state.menu.sort_dropdown.list_state.selected = Some(0);
            update_render_list(env);
        }
        MENU_SORT_DROPDOWN_PRIO => {
            env.ui_state.menu.sort_dropdown.expanded = !env.ui_state.menu.sort_dropdown.expanded;
            env.ui_state.menu.sort_dropdown.list_state.selected = Some(1);
            update_render_list(env);
        }
        MENU_SORT_DROPDOWN_INCEPTION => {
            env.ui_state.menu.sort_dropdown.expanded = !env.ui_state.menu.sort_dropdown.expanded;
            env.ui_state.menu.sort_dropdown.list_state.selected = Some(2);
            update_render_list(env);
        }
        MENU_SORT_DROPDOWN_COMPLETION => {
            env.ui_state.menu.sort_dropdown.expanded = !env.ui_state.menu.sort_dropdown.expanded;
            env.ui_state.menu.sort_dropdown.list_state.selected = Some(3);
            update_render_list(env);
        }

        _ => {}
    }
    Focus::None
}
