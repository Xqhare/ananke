use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Text, traits::Widget},
};

use crate::{render::header::file::render_header_file_menu_button, startup::Environment};

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
