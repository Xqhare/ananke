use std::collections::BTreeMap;

use talos::{
    atlases::LayoutAtlas,
    codex::Codex,
    layout::Rect,
    render::Canvas,
    widgets::{
        Block,
        stateful::{BlockBox, Button, InnerBorder, Sequence, Table, TextBox},
        traits::Widget,
    },
};

use crate::{keys::LIST_RECT, startup::Environment, state::list::TaskState};

pub fn render_list(
    canvas: &mut Canvas,
    codex: &Codex,
    layout_atlas: &LayoutAtlas,
    _clickable_regions: &mut BTreeMap<String, Rect>,
    env: &mut Environment,
) {
    let area = layout_atlas.get_known_rect(LIST_RECT);
    let default_style = env.styles.get_default();
    let ok_style = env.styles.get_ok();
    let error_style = env.styles.get_error();

    // Let me be honest here. I don't really like the below code, but it is the only way I found to
    // make it work.

    let Environment {
        render_tasks,
        ui_state,
        ..
    } = env;

    let mut states_map: BTreeMap<usize, &mut TaskState> = ui_state
        .dynamic_states
        .iter_mut()
        .map(|(k, v)| (*k, v))
        .collect();

    let mut button_storage: Vec<Vec<Button>> = render_tasks.iter().map(|_| Vec::new()).collect();
    // Stores a tuple of (inception, creation) dates
    let mut dates_storage: Vec<Vec<((BlockBox, TextBox, Block), (BlockBox, TextBox, Block))>> =
        render_tasks.iter().map(|_| Vec::new()).collect();
    let mut text_fields: Vec<Vec<TextBox>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut tags: Vec<Vec<BlockBox>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut rows: Vec<Vec<Sequence>> = Vec::new();

    for ((((task, buttons_vec), dates_vec), text_fields_vec), tags_vec) in render_tasks
        .iter()
        .zip(button_storage.iter_mut())
        .zip(dates_storage.iter_mut())
        .zip(text_fields.iter_mut())
        .zip(tags.iter_mut())
    {
        if let Some(state) = states_map.remove(&task.id()) {
            let mut row = vec![];

            let done_button_text = if task.is_done() { "Done" } else { "To Do" };
            buttons_vec.push(
                Button::new(done_button_text, &mut state.done_button, codex).with_style(ok_style),
            );
            buttons_vec.push(
                Button::new("Delete", &mut state.delete_button, codex).with_style(error_style),
            );

            let column0 = Sequence::new(state.generic_sequence, buttons_vec.iter_mut()).vertical();
            row.push(column0);

            let mut inception_block = Block::new().with_fat_border().with_style(default_style);
            let mut inception_text =
                TextBox::new(&mut state.inception_textbox).with_style(default_style);
            let box_inception = BlockBox::new(&mut inception_block, &mut inception_text);
            dates_vec.push(box_inception);
            let column1 = Sequence::new(state.generic_sequence, dates_vec.iter_mut()).vertical();
            row.push(column1);

            rows.push(row);
        }
    }

    Table::new(&mut ui_state.task_table)
        .with_rows(rows.iter_mut())
        .with_border_style(default_style)
        .with_style(default_style)
        .draw_inner_border(InnerBorder::Rows)
        .render(canvas, area, codex);
}
