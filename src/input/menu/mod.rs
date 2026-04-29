use crate::{
    input::{Focus, creator::update_render_list},
    keys::{
        MENU_SHOW_DROPDOWN, MENU_SHOW_DROPDOWN_ALL, MENU_SHOW_DROPDOWN_DONE,
        MENU_SHOW_DROPDOWN_OPEN, MENU_SHOW_DROPDOWN_STATE,
    },
    startup::Environment,
    utils::toggle_dropdown,
};

pub fn handle_menu_mouse(env: &mut Environment, name: &str) -> Focus {
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
        _ => {}
    }
    Focus::None
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
