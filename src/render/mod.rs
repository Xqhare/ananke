use std::collections::BTreeMap;

use talos::{atlases::LayoutAtlas, codex::Codex, layout::Rect, render::Canvas};

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
    render_header(
        canvas,
        codex,
        layout_atlas,
        clickable_regions,
        last_frame_dur,
        env,
    );
}
