use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{
        Block, Text,
        stateful::{BlockBox, Button, Sequence, SequenceState, TextBox},
        traits::Widget,
    },
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
    clickable_regions: &mut BTreeMap<String, Rect>,
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
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    render_row0(canvas, codex, layout_atlas, env, clickable_regions);
    render_row1(canvas, codex, layout_atlas, env, clickable_regions);
    render_row2(canvas, codex, layout_atlas, env);
    render_row3_buttons(canvas, codex, layout_atlas, env, clickable_regions);
}

fn render_row0(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &mut Environment,
    clickable_regions: &mut BTreeMap<String, Rect>,
) {
    let rect0 = layout_atlas.get_known_rect("creator_textbox_prio");
    let rect01 = layout_atlas.get_known_rect("creator_prio_text");
    let rect1 = layout_atlas.get_known_rect("creator_textbox_task");
    let style = env.styles.get_default();
    let highlight_style = env.styles.get_known_style("cursor");

    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            "creator_task_prio_entry_textbox_state" => {
                state0 = state.as_text_box_mut();
            }
            "creator_task_entry_textbox_state" => {
                state1 = state.as_text_box_mut();
            }
            _ => {}
        }
    }

    let state0 = state0.unwrap();
    let state1 = state1.unwrap();

    let mut textbox0 = TextBox::new(state0)
        .with_style(style)
        .with_highlight_style(highlight_style);

    let mut block = Block::new().with_style(style);
    let mut blockbox0 = BlockBox::new(&mut block, &mut textbox0).with_style(style);
    let mut text0 = Text::new("Priority:", codex)
        .align_center()
        .align_vertically()
        .with_style(style);

    blockbox0.render(canvas, rect0, codex);
    text0.render(canvas, rect01, codex);

    let mut textbox1 = TextBox::new(state1)
        .with_style(style)
        .with_highlight_style(highlight_style);

    let mut block = Block::new().with_style(style);
    let mut blockbox1 = BlockBox::new(&mut block, &mut textbox1).with_style(style);

    blockbox1.render(canvas, rect1, codex);

    clickable_regions.insert("creator_textbox_task".to_string(), rect0);
    clickable_regions.insert("creator_textbox_prio".to_string(), rect1);
}

fn render_row1(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &mut Environment,
    clickable_regions: &mut BTreeMap<String, Rect>,
) {
    let mut rect0 = layout_atlas.get_known_rect("creator_textbox_inception");
    let mut rect1 = layout_atlas.get_known_rect("creator_text_context_tags");
    // Some final minor adjustments to make sure the boxes line up correctly
    rect0.x += 1;
    rect0.width -= 2;
    rect1.x -= 1;
    let style = env.styles.get_default();
    let highlight_style = env.styles.get_known_style("cursor");

    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            "creator_task_creation_date_entry_textbox_state" => {
                state0 = state.as_text_box_mut();
            }
            "creator_text_context_tags" => {
                state1 = state.as_text_box_mut();
            }
            _ => {}
        }
    }

    let state0 = state0.unwrap();
    let state1 = state1.unwrap();

    let mut textbox0 = TextBox::new(state0)
        .with_style(style)
        .with_highlight_style(highlight_style);

    let mut block = Block::new().with_style(style);
    let mut blockbox0 = BlockBox::new(&mut block, &mut textbox0).with_style(style);

    blockbox0.render(canvas, rect0, codex);

    let mut text1 = Text::new(state1.text.get_content(), codex).align_center();
    let mut block = Block::new().with_style(style);
    let mut blockbox1 = BlockBox::new(&mut block, &mut text1).with_style(style);

    blockbox1.render(canvas, rect1, codex);

    clickable_regions.insert("creator_textbox_inception".to_string(), rect0);
}

fn render_row2(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &mut Environment,
) {
    let mut rect0 = layout_atlas.get_known_rect("creator_text_project_tags");
    let mut rect1 = layout_atlas.get_known_rect("creator_text_special_tags");
    // Some final minor adjustments to make sure the boxes line up correctly
    rect0.x += 1;
    rect0.width -= 2;
    rect1.x -= 1;
    let style = env.styles.get_default();
    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            "creator_text_project_tags" => {
                state0 = state.as_text_box_mut();
            }
            "creator_text_special_tags" => {
                state1 = state.as_text_box_mut();
            }
            _ => {}
        }
    }

    let state0 = state0.unwrap();
    let state1 = state1.unwrap();

    let mut text0 = Text::new(state0.text.get_content(), codex)
        .align_center()
        .with_style(style);
    let mut block = Block::new().with_style(style);
    let mut blockbox0 = BlockBox::new(&mut block, &mut text0).with_style(style);

    blockbox0.render(canvas, rect0, codex);

    let mut text1 = Text::new(state1.text.get_content(), codex)
        .align_center()
        .with_style(style);
    let mut block = Block::new().with_style(style);
    let mut blockbox1 = BlockBox::new(&mut block, &mut text1).with_style(style);

    blockbox1.render(canvas, rect1, codex);
}

fn render_row3_buttons(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &mut Environment,
    clickable_regions: &mut BTreeMap<String, Rect>,
) {
    let rect0 = layout_atlas.get_known_rect("creator_button_reset_new_task");
    let rect1 = layout_atlas.get_known_rect("creator_button_add_new_task");
    let style = env.styles.get_default();
    let highlight_style = env.styles.get_known_style("cursor");

    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            "creator_task_save_button_state" => {
                state0 = state.as_button_mut();
            }
            "creator_task_forget_button_state" => {
                state1 = state.as_button_mut();
            }
            _ => {}
        }
    }

    let state0 = state0.unwrap();
    let state1 = state1.unwrap();

    let mut button0 = Button::new("Save", state0, codex)
        .with_clicked_style(highlight_style)
        .with_style(style);

    let mut button1 = Button::new("Clear", state1, codex)
        .with_clicked_style(highlight_style)
        .with_style(style);

    button0.render(canvas, rect0, codex);
    button1.render(canvas, rect1, codex);

    clickable_regions.insert("creator_button_reset_new_task".to_string(), rect0);
    clickable_regions.insert("creator_button_add_new_task".to_string(), rect1);
}
