use std::collections::BTreeMap;

use talos::{atlases::LayoutAtlas, layout::Rect, render::Canvas};

/// Renders the application
///
/// # Arguments
/// * `canvas` - The canvas to render to
/// * `layout_atlas` - The layout atlas for the current frame
///
/// # Returns
/// A map of all clickable regions for input processing
pub fn render_app(
    canvas: &mut Canvas,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    last_frame_dur: u128,
    path_amount: usize,
) -> BTreeMap<String, Rect> {
    todo!()
}
