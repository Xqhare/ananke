use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Area, traits::Widget},
};

use crate::{
    render::{creator::render_creator, header::render_header, menu::render_menu},
    startup::Environment,
};

mod creator;
mod header;
mod menu;

/// Renders the application
///
/// # Arguments
/// * `canvas` - The canvas to render to
/// * `layout_atlas` - The layout atlas for the current frame
/// * `clickable_regions` - The clickable regions for the current frame
/// * `last_frame_dur` - The duration of the last frame
/// * `env` - The environment
///
/// The clickable regions are updated with the new clickable regions of the current rendered frame
pub fn render_app(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    last_frame_dur: u128,
    env: &mut Environment,
) {
    clickable_regions.clear();
    let entire_canvas = canvas.size_rect();
    let default_style = env.styles.get_default();
    // TODO: Remove `Area` below and see exactly where I need to draw the BG manually to save on draw calls
    Area::new()
        .with_style(default_style)
        .render(canvas, entire_canvas, codex);
    render_creator(canvas, codex, layout_atlas, clickable_regions, env);
    // Render header after creator to ensure its buttons are on top
    render_header(
        canvas,
        codex,
        layout_atlas,
        clickable_regions,
        last_frame_dur,
        env,
    );
    render_menu(canvas, codex, layout_atlas, clickable_regions, env);
}
