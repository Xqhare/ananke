use std::collections::BTreeMap;

use talos::{
    LayoutBuilder,
    atlases::LayoutAtlas,
    codex::Codex,
    layout::{Constraint, Rect},
    render::Canvas,
    widgets::{
        Block, Text,
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
    // make it work. (Without reworking `Talos` a fair bit)
    //
    // To keep it short: A lot of code is needed for heap pointer management.

    // --------------------- said heap pointers ---------------------
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

    let mut dates_inception_textbox_storage: Vec<Vec<TextBox>> =
        render_tasks.iter().map(|_| Vec::new()).collect();
    let mut dates_inception_block_storage: Vec<Vec<Block>> =
        render_tasks.iter().map(|_| Vec::new()).collect();
    let mut dates_completion_textbox_storage: Vec<Vec<TextBox>> =
        render_tasks.iter().map(|_| Vec::new()).collect();
    let mut dates_completion_block_storage: Vec<Vec<Block>> =
        render_tasks.iter().map(|_| Vec::new()).collect();

    let mut prio_text_storage: Vec<Vec<Text>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut prio_textbox_storage: Vec<Vec<TextBox>> =
        render_tasks.iter().map(|_| Vec::new()).collect();
    let mut prio_block_storage: Vec<Vec<Block>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut prio_blockbox_storage: Vec<Vec<BlockBox>> =
        render_tasks.iter().map(|_| Vec::new()).collect();
    let mut prio_vec: Vec<Vec<BlockBox>> = render_tasks.iter().map(|_| Vec::new()).collect();

    let mut button_storage: Vec<Vec<Button>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut dates_storage: Vec<Vec<BlockBox>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut text_textbox: Vec<Vec<TextBox>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut tags: Vec<Vec<BlockBox>> = render_tasks.iter().map(|_| Vec::new()).collect();
    let mut rows: Vec<Vec<Sequence>> = Vec::new();
    let mut col2: Vec<Vec<Sequence>> = render_tasks.iter().map(|_| Vec::new()).collect();

    // --------------------- end of heap pointer alloc ---------------------

    for (
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (((task, buttons_vec), prio_textbox_vec), prio_block_vec),
                                        prio_vec,
                                    ),
                                    dates_inception_textbox_vec,
                                ),
                                dates_inception_block_vec,
                            ),
                            dates_completion_textbox_vec,
                        ),
                        dates_completion_block_vec,
                    ),
                    dates_vec,
                ),
                text_textbox_vec,
            ),
            tags_vec,
        ),
        col2_vec,
    ) in render_tasks
        .iter()
        .zip(button_storage.iter_mut())
        .zip(prio_textbox_storage.iter_mut())
        .zip(prio_block_storage.iter_mut())
        .zip(prio_vec.iter_mut())
        .zip(dates_inception_textbox_storage.iter_mut())
        .zip(dates_inception_block_storage.iter_mut())
        .zip(dates_completion_textbox_storage.iter_mut())
        .zip(dates_completion_block_storage.iter_mut())
        .zip(dates_storage.iter_mut())
        .zip(text_textbox.iter_mut())
        .zip(tags.iter_mut())
        .zip(col2.iter_mut())
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

            let col0 = Sequence::new(state.generic_sequence, buttons_vec.iter_mut()).vertical();
            row.push(col0);

            prio_textbox_vec.push(TextBox::new(&mut state.prio_textbox).with_style(default_style));
            prio_block_vec.push(
                Block::new()
                    .with_fat_border()
                    .with_style(default_style)
                    .with_bg_fill(),
            );
            prio_vec.push(BlockBox::new(
                prio_block_vec.last_mut().unwrap(),
                prio_textbox_vec.last_mut().unwrap(),
            ));

            let col1 = Sequence::new(state.generic_sequence, prio_vec.iter_mut()).vertical();
            row.push(col1);

            dates_inception_textbox_vec
                .push(TextBox::new(&mut state.inception_textbox).with_style(default_style));
            dates_inception_block_vec.push(
                Block::new()
                    .with_fat_border()
                    .with_style(default_style)
                    .with_bg_fill(),
            );
            dates_vec.push(BlockBox::new(
                dates_inception_block_vec.last_mut().unwrap(),
                dates_inception_textbox_vec.last_mut().unwrap(),
            ));

            dates_completion_textbox_vec
                .push(TextBox::new(&mut state.completion_textbox).with_style(default_style));
            dates_completion_block_vec.push(
                Block::new()
                    .with_fat_border()
                    .with_style(default_style)
                    .with_bg_fill(),
            );
            dates_vec.push(BlockBox::new(
                dates_completion_block_vec.last_mut().unwrap(),
                dates_completion_textbox_vec.last_mut().unwrap(),
            ));

            col2_vec.push(Sequence::new(state.generic_sequence, dates_vec.iter_mut()).vertical());

            row.push(Sequence::new(state.generic_sequence, col2_vec.iter_mut()));
            rows.push(row);
        }
    }

    let layout = LayoutBuilder::new()
        .add_constraint(Constraint::Length(10))
        .add_constraint(Constraint::Length(5))
        .add_constraint(Constraint::Max(25))
        .add_constraint(Constraint::Min(1))
        .build();

    Table::new(&mut ui_state.task_table)
        .with_rows(rows.iter_mut())
        .with_border_style(default_style)
        .with_style(default_style)
        .draw_inner_border(InnerBorder::Rows)
        .with_col_layout(layout)
        .with_row_height(6)
        .render(canvas, area, codex);
}
