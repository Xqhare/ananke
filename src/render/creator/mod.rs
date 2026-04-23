use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{Block, traits::Widget},
};

use crate::{render::creator::help::render_help, startup::Environment};

mod help;

pub fn render_creator(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    render_outer_block(canvas, codex, layout_atlas, clickable_regions, env);
}

fn render_outer_block(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let area = layout_atlas.get_known_rect("creator_rect");
    let style = env.styles.get_default();
    let mut block = Block::new().with_fat_border().with_style(style);
    block.render(canvas, area, codex);
    if env
        .states
        .get("header_help_menu_button_state")
        .unwrap()
        .as_button()
        .unwrap()
        .clicked
    {
        render_help(canvas, codex, layout_atlas, env);
    } else {
        render_inner_creator(canvas, codex, layout_atlas, clickable_regions, env);
    }
}

fn render_inner_creator(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &BTreeMap<String, Rect>,
    env: &mut Environment,
) {
}
