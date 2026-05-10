use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Area, traits::Widget},
};

use crate::{
    render::{
        creator::render_creator, header::render_header, list::render_list, menu::render_menu,
    },
    startup::Environment,
};

mod creator;
mod header;
mod list;
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
    // While the Area below does double the rendering of the entire canvas, (drawing the bg twice) it does
    // ensure no missed 'pixels' and simplifies the code.
    // Hundreds of fps is fine for a to-do app.
    Area::new()
        .with_style(default_style)
        .render(canvas, entire_canvas, codex);

    render_creator(canvas, codex, layout_atlas, clickable_regions, env);
    // Render header after creator to ensure its dropdown buttons are on top
    render_header(
        canvas,
        codex,
        layout_atlas,
        clickable_regions,
        last_frame_dur,
        env,
    );
    render_list(canvas, codex, layout_atlas, clickable_regions, env);
    // Render menu after list to ensure its dropdown buttons are on top
    render_menu(canvas, codex, layout_atlas, clickable_regions, env);
}
