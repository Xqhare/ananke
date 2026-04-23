use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Block, traits::Widget},
};

use crate::startup::Environment;

pub fn render_menu(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let area = layout_atlas.get_known_rect("menu_rect");
    let style = env.styles.get_default();
    let mut block = Block::new().with_style(style);
    block.render(canvas, area, codex);
}
