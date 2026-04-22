use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Area, traits::Widget},
};

use crate::{render::header::render_header, startup::Environment};

mod header;

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
    render_header(
        canvas,
        codex,
        layout_atlas,
        clickable_regions,
        last_frame_dur,
        env,
    );
}
