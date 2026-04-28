use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Text, stateful::Button, traits::Widget},
};

use crate::{
    keys::{
        HEADER_EXIT_BUTTON, HEADER_EXIT_BUTTON_STATE, HEADER_FILE_PATH, HEADER_FPS,
        HEADER_HELP_BUTTON, HEADER_HELP_BUTTON_STATE, HEADER_SAVE_BUTTON, HEADER_SAVE_BUTTON_STATE,
        styles::DEFAULT_INVERTED,
    },
    render::header::file::render_header_file_menu_button,
    startup::Environment,
};

mod file;

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
    render_header_save_button(canvas, codex, layout_atlas, clickable_regions, env);
    render_header_help_button(canvas, codex, layout_atlas, clickable_regions, env);
    render_header_exit_button(canvas, codex, layout_atlas, clickable_regions, env);
}

fn render_header_help_button(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let rect = layout_atlas.get_known_rect(HEADER_HELP_BUTTON);
    let default_style = env.styles.get_default();
    let default_clicked_style = env.styles.get_known_style(DEFAULT_INVERTED);
    let help_button_state = &mut env
        .states
        .get_mut(HEADER_HELP_BUTTON_STATE)
        .unwrap()
        .as_button_mut()
        .unwrap();
    let mut button = Button::new("Help", help_button_state, codex)
        .with_style(default_style)
        .with_clicked_style(default_clicked_style);
    button.render(canvas, rect, codex);
    clickable_regions.insert(HEADER_HELP_BUTTON.to_string(), rect);
}

fn render_header_exit_button(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let rect = layout_atlas.get_known_rect(HEADER_EXIT_BUTTON);
    let default_style = env.styles.get_default();
    let exit_button_state = &mut env
        .states
        .get_mut(HEADER_EXIT_BUTTON_STATE)
        .unwrap()
        .as_button_mut()
        .unwrap();
    let mut button = Button::new("Exit", exit_button_state, codex).with_style(default_style);
    button.render(canvas, rect, codex);
    clickable_regions.insert(HEADER_EXIT_BUTTON.to_string(), rect);
}

fn render_header_save_button(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let rect = layout_atlas.get_known_rect(HEADER_SAVE_BUTTON);
    let default_style = env.styles.get_default();
    let save_button_state = &mut env
        .states
        .get_mut(HEADER_SAVE_BUTTON_STATE)
        .unwrap()
        .as_button_mut()
        .unwrap();
    let mut button = Button::new("Save", save_button_state, codex).with_style(default_style);
    button.render(canvas, rect, codex);
    clickable_regions.insert(HEADER_SAVE_BUTTON.to_string(), rect);
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
            1_000_000 / last_frame_dur
        }
    };
    let rect = layout_atlas.get_known_rect(HEADER_FPS);
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
    let rect = layout_atlas.get_known_rect(HEADER_FILE_PATH);
    let default_style = env.styles.get_default();
    let path = env.list.get_path();
    let mut text = Text::new(format!("Current file: {}", path.display()), codex)
        .align_center()
        .align_vertically()
        .with_style(default_style);
    text.render(canvas, rect, codex);
}
