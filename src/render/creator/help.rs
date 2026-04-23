use std::collections::BTreeMap;

use talos::{
    LayoutBuilder,
    atlases::LayoutAtlas,
    codex::Codex,
    layout::{Constraint, Direction, Rect},
    render::Canvas,
    widgets::{Block, Text, traits::Widget},
};

use crate::startup::Environment;

pub fn render_help(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &Environment,
) {
    render_about(canvas, codex, layout_atlas, env);
    render_help_block(canvas, codex, layout_atlas, env);
}

fn render_help_block(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &Environment,
) {
    let area = layout_atlas.get_known_rect("creator_help_page_left");
    let style = env.styles.get_default();
    let mut block = Block::default()
        .title("Help", codex, false)
        .with_style(style)
        .with_beautify_border_breaks();
    block.render(canvas, area, codex);
    let area = block.inner(area);
    render_inner_help(canvas, codex, area, env);
}

fn render_inner_help(canvas: &mut Canvas, codex: &Codex, area: Rect, env: &Environment) {
    let style = env.styles.get_default();
    let area = area.shrink(1, 1);
    let layout = LayoutBuilder::new()
        .direction(Direction::Horizontal)
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .add_constraint(Constraint::Min(1))
        .build()
        .split(area);

    let mut left_text = Text::new("Welcome to Ananke!\nThis is a todo list manager.\nIt uses the 'todo.txt' format.\n \nThe currently loaded file is shown in the top right.\nLeft of it is the current FPS.\nThis is followed by the 'Exit' button. This not only exits Ananke, but also saves the state of the list.", codex).with_style(style);
    left_text.render(canvas, layout[0], codex);
    let mut middle_text = Text::new("The 'Help' Button shows this text, and the 'Save' button to the left of it saves the current state of the list.\n\nThe 'File' button on the very left is used to load, create or forget todo lists.\nAnanke will never delete a list from your system under any circumstances.", codex).with_style(style);
    middle_text.render(canvas, layout[1], codex);
    let mut right_text = Text::new("To load an existing list, import it by clicking on the 'File' Button, then clicking on the 'New' Button and entering the full path to the file in the text field. Hit enter to load.", codex).with_style(style);
    right_text.render(canvas, layout[2], codex);
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");
const SOURCE: &str = env!("CARGO_PKG_REPOSITORY");
const TODO: &str = "https://github.com/todotxt/todo.txt";

fn render_about(canvas: &mut Canvas, codex: &Codex, layout_atlas: &LayoutAtlas, env: &Environment) {
    let area = layout_atlas.get_known_rect("creator_help_page_right");
    let style = env.styles.get_default();
    let mut block = Block::default()
        .title("About", codex, false)
        .with_style(style)
        .with_beautify_border_breaks();
    block.render(canvas, area, codex);
    let area = block.inner(area);
    let mut text = Text::new(format!(" Version: {} \n Author: {} \n License: {} \n \n Source code is available at: {} \n \n Ananke uses the 'todo.txt' format as specified in: \n {}", VERSION, AUTHOR, LICENSE, SOURCE, TODO), codex)
        .with_style(style)
        .align_vertically();
    text.render(canvas, area, codex);
}
