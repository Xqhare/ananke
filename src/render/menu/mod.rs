use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{
        Block, Text,
        stateful::{BlockBox, Dropdown, DropdownState},
        traits::Widget,
    },
};

use crate::{
    keys::{
        MENU_RECT, MENU_SHOW_DROPDOWN, MENU_SHOW_DROPDOWN_ALL, MENU_SHOW_DROPDOWN_DONE,
        MENU_SHOW_DROPDOWN_OPEN, MENU_SHOW_DROPDOWN_STATE, MENU_SHOW_DROPDOWN_TEXT,
        styles::DEFAULT_INVERTED,
    },
    startup::Environment,
};

pub fn render_menu(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let area = layout_atlas.get_known_rect(MENU_RECT);
    let style = env.styles.get_default();
    let mut block = Block::new().with_style(style);
    block.render(canvas, area, codex);
    render_show_dropdown(canvas, codex, layout_atlas, clickable_regions, env);
    render_sort_dropdown(canvas, codex, layout_atlas, clickable_regions, env);
    render_prio_search(canvas, codex, layout_atlas, clickable_regions, env);
    render_text_search(canvas, codex, layout_atlas, clickable_regions, env);
}

pub fn render_show_dropdown(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let rect_text = layout_atlas.get_known_rect(MENU_SHOW_DROPDOWN_TEXT);
    let rect_dropdown = layout_atlas.get_known_rect(MENU_SHOW_DROPDOWN);
    let style = env.styles.get_default();
    let inv_style = env.styles.get_known_style(DEFAULT_INVERTED);

    let mut text = Text::new("Show:", codex)
        .with_style(style)
        .align_center()
        .align_vertically();
    text.render(canvas, rect_text, codex);

    let mut dropdown_state = env
        .states
        .get_mut(MENU_SHOW_DROPDOWN_STATE)
        .unwrap()
        .as_dropdown_mut()
        .unwrap();
    let selected = dropdown_state.list_state.selected;
    let mut items = vec![
        Text::new("All", codex).align_center().with_style(style),
        Text::new("Done", codex).align_center().with_style(style),
        Text::new("Open", codex).align_center().with_style(style),
    ];
    let mut dropdown = Dropdown::new(&mut dropdown_state, items.iter_mut())
        .with_style(style)
        .with_active_style(style)
        .with_selected_style(inv_style)
        .with_fat_border();
    if let Some(selected) = selected {
        match selected {
            0 => dropdown = dropdown.with_label("All"),
            1 => dropdown = dropdown.with_label("Done"),
            2 => dropdown = dropdown.with_label("Open"),
            _ => unreachable!(),
        }
    };
    dropdown.render(canvas, rect_dropdown, codex);

    clickable_regions.insert(MENU_SHOW_DROPDOWN.to_string(), rect_dropdown);

    if dropdown_state.expanded {
        clickable_regions.insert(
            MENU_SHOW_DROPDOWN_ALL.to_string(),
            Rect::new(
                rect_dropdown.x,
                rect_dropdown.y.saturating_add(rect_dropdown.height * 1),
                rect_dropdown.width,
                rect_dropdown.height,
            ),
        );
        clickable_regions.insert(
            MENU_SHOW_DROPDOWN_DONE.to_string(),
            Rect::new(
                rect_dropdown.x,
                rect_dropdown.y.saturating_add(rect_dropdown.height * 2),
                rect_dropdown.width,
                rect_dropdown.height,
            ),
        );
        clickable_regions.insert(
            MENU_SHOW_DROPDOWN_OPEN.to_string(),
            Rect::new(
                rect_dropdown.x,
                rect_dropdown.y.saturating_add(rect_dropdown.height * 3),
                rect_dropdown.width,
                rect_dropdown.height,
            ),
        );
    }
}

// Sort by prio, inception or completion date
pub fn render_sort_dropdown(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
}

pub fn render_prio_search(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
}

pub fn render_text_search(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
}
