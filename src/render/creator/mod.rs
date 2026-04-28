use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{
        Block, Text,
        stateful::{BlockBox, Button, TextBox},
        traits::Widget,
    },
};

use crate::{
    keys::{
        CREATOR_CLEAR_BUTTON, CREATOR_CLEAR_BUTTON_STATE, CREATOR_INCEPTION_ENTRY_TEXTBOX,
        CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE, CREATOR_INCEPTION_TEXT, CREATOR_PRIO_ENTRY_TEXTBOX,
        CREATOR_PRIO_ENTRY_TEXTBOX_STATE, CREATOR_PRIO_TEXT, CREATOR_RECT, CREATOR_SAVE_BUTTON,
        CREATOR_SAVE_BUTTON_STATE, CREATOR_TASK_ENTRY_TEXTBOX, CREATOR_TASK_ENTRY_TEXTBOX_STATE,
        CREATOR_TEXT_CONTEXT_TAGS, CREATOR_TEXT_PROJECT_TAGS, CREATOR_TEXT_SPECIAL_TAGS,
        HEADER_HELP_BUTTON_STATE, styles::CURSOR,
    },
    render::creator::help::render_help,
    startup::Environment,
};

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
    let area = layout_atlas.get_known_rect(CREATOR_RECT);
    let style = env.styles.get_default();
    let mut block = Block::new().with_style(style);
    block.render(canvas, area, codex);
    if env
        .states
        .get(HEADER_HELP_BUTTON_STATE)
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
    let rect0 = layout_atlas.get_known_rect(CREATOR_PRIO_ENTRY_TEXTBOX);
    let rect01 = layout_atlas.get_known_rect(CREATOR_PRIO_TEXT);
    let rect1 = layout_atlas.get_known_rect(CREATOR_TASK_ENTRY_TEXTBOX);
    let rect10 = layout_atlas.get_known_rect(CREATOR_INCEPTION_TEXT);
    let style = env.styles.get_default();
    let highlight_style = env.styles.get_known_style(CURSOR);

    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            CREATOR_PRIO_ENTRY_TEXTBOX_STATE => {
                state0 = state.as_text_box_mut();
            }
            CREATOR_TASK_ENTRY_TEXTBOX_STATE => {
                state1 = state.as_text_box_mut();
            }
            _ => {}
        }
    }

    let state0 = state0.unwrap();
    let state1 = state1.unwrap();

    let hint_text0 = Text::new("[A-Z]", codex)
        .align_center()
        .align_vertically()
        .with_style(style);
    let mut textbox0 = TextBox::new(state0)
        .with_style(style)
        .with_highlight_style(highlight_style)
        .with_hint_text(hint_text0);

    let mut block = Block::new().with_style(style).with_fat_border();
    let mut blockbox0 = BlockBox::new(&mut block, &mut textbox0).with_style(style);
    let mut text0 = Text::new("Prio:", codex)
        .align_center()
        .align_vertically()
        .with_style(style);

    blockbox0.render(canvas, rect0, codex);
    text0.render(canvas, rect01, codex);

    let hint_text1 = Text::new(
        "Enter a task with tags (@context, +project, special_key:value_tag) here!",
        codex,
    )
    .align_center()
    .align_vertically()
    .with_style(style);
    let mut textbox1 = TextBox::new(state1)
        .with_style(style)
        .with_highlight_style(highlight_style)
        .with_hint_text(hint_text1);

    let mut block = Block::new().with_style(style).with_fat_border();
    let mut blockbox1 = BlockBox::new(&mut block, &mut textbox1).with_style(style);
    let mut text1 = Text::new("Creation Date:", codex)
        .align_center()
        .align_vertically()
        .with_style(style);

    text1.render(canvas, rect10, codex);

    blockbox1.render(canvas, rect1, codex);

    clickable_regions.insert(CREATOR_TASK_ENTRY_TEXTBOX.to_string(), rect1);
    clickable_regions.insert(CREATOR_PRIO_ENTRY_TEXTBOX.to_string(), rect0);
}

fn render_row1(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &mut Environment,
    clickable_regions: &mut BTreeMap<String, Rect>,
) {
    let rect0 = layout_atlas.get_known_rect(CREATOR_INCEPTION_ENTRY_TEXTBOX);
    let rect1 = layout_atlas.get_known_rect(CREATOR_TEXT_CONTEXT_TAGS);
    let style = env.styles.get_default();
    let highlight_style = env.styles.get_known_style(CURSOR);

    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            CREATOR_INCEPTION_ENTRY_TEXTBOX_STATE => {
                state0 = state.as_text_box_mut();
            }
            CREATOR_TEXT_CONTEXT_TAGS => {
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

    let mut block = Block::new().with_style(style).with_fat_border();
    let mut blockbox0 = BlockBox::new(&mut block, &mut textbox0).with_style(style);

    blockbox0.render(canvas, rect0, codex);

    let mut text1 = Text::new(state1.text.get_content(), codex).align_center();
    let mut block = Block::new().with_style(style);
    let mut blockbox1 = BlockBox::new(&mut block, &mut text1).with_style(style);

    blockbox1.render(canvas, rect1, codex);

    clickable_regions.insert(CREATOR_INCEPTION_ENTRY_TEXTBOX.to_string(), rect0);
}

fn render_row2(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    env: &mut Environment,
) {
    let rect0 = layout_atlas.get_known_rect(CREATOR_TEXT_PROJECT_TAGS);
    let rect1 = layout_atlas.get_known_rect(CREATOR_TEXT_SPECIAL_TAGS);
    let style = env.styles.get_default();
    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            CREATOR_TEXT_PROJECT_TAGS => {
                state0 = state.as_text_box_mut();
            }
            CREATOR_TEXT_SPECIAL_TAGS => {
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
    let rect0 = layout_atlas.get_known_rect(CREATOR_SAVE_BUTTON);
    let rect1 = layout_atlas.get_known_rect(CREATOR_CLEAR_BUTTON);
    let style0 = env.styles.get_ok();
    let style1 = env.styles.get_warning();
    let highlight_style = env.styles.get_known_style(CURSOR);

    let mut state0 = None;
    let mut state1 = None;

    for (name, state) in env.states.iter_mut() {
        match name.as_str() {
            CREATOR_SAVE_BUTTON_STATE => {
                state0 = state.as_button_mut();
            }
            CREATOR_CLEAR_BUTTON_STATE => {
                state1 = state.as_button_mut();
            }
            _ => {}
        }
    }

    let state0 = state0.unwrap();
    let state1 = state1.unwrap();

    let mut button0 = Button::new("Save", state0, codex)
        .with_clicked_style(highlight_style)
        .with_style(style0);

    let mut button1 = Button::new("Clear", state1, codex)
        .with_clicked_style(highlight_style)
        .with_style(style1);

    button0.render(canvas, rect0, codex);
    button1.render(canvas, rect1, codex);

    clickable_regions.insert(CREATOR_CLEAR_BUTTON.to_string(), rect1);
    clickable_regions.insert(CREATOR_SAVE_BUTTON.to_string(), rect0);
}
