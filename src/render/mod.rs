//! # UI Composition & Rendering
//!
//! This module orchestrates the rendering phase of the application. 
//!
//! In **Talos**, rendering is "atomic": the entire UI is redrawn every frame based 
//! on the current `UiState` and `LayoutAtlas`. 

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

pub mod creator;
pub mod header;
pub mod list;
pub mod menu;

/// The root rendering function.
///
/// It performs the following steps:
/// 1. Clears the `clickable_regions` map for the new frame.
/// 2. Draws the background `Area`.
/// 3. Sequentially calls sub-renderers for each component.
///
/// Note: The order of calls matters for Z-indexing (elements rendered later appear on top).
pub fn render_app(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    last_frame_dur: u128,
    env: &mut Environment,
) {
    // Reset click tracking. As we render widgets, they will register their Rects here.
    clickable_regions.clear();
    
    let entire_canvas = canvas.size_rect();
    let default_style = env.styles.get_default();

    // Fill the background.
    Area::new()
        .with_style(default_style)
        .render(canvas, entire_canvas, codex);

    // Component Rendering
    render_creator(canvas, codex, layout_atlas, clickable_regions, env);
    
    // The header is rendered after the creator to ensure dropdown menus overlap the creator.
    render_header(
        canvas,
        codex,
        layout_atlas,
        clickable_regions,
        last_frame_dur,
        env,
    );
    
    render_list(canvas, codex, layout_atlas, clickable_regions, env);
    
    // The menu is rendered last for similar overlap reasons.
    render_menu(canvas, codex, layout_atlas, clickable_regions, env);
}
