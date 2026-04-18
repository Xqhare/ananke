use std::{
    collections::BTreeMap,
    thread::sleep,
    time::{Duration, Instant},
};

use talos::layout::Rect;

use crate::{
    error::AnankeResult, input::process_input, layout::make_frame_layout, render::render_app,
    startup::startup, state::make_state, utils::fps_sleeper,
};

mod error;
mod input;
mod layout;
mod render;
mod startup;
mod state;
mod utils;

fn main() -> AnankeResult<()> {
    let (env, mut talos, gen_layout, path_amount, mut state) = startup()?;

    let mut clickable_regions: BTreeMap<String, Rect> = BTreeMap::new();

    let mut run = true;
    let mut last_frame = Instant::now();
    let mut last_frame_dur = 0;

    while run {
        talos.begin_frame();
        let (canvas, _) = talos.render_ctx();
        let frame_layout = make_frame_layout(&canvas.size_rect(), &gen_layout);

        // TODO: Consider the ordering of rendering and processing
        // Could also process first and use the region of last frame to process clicks
        //
        // Lets keep it this way until I know why its stupid
        clickable_regions = render_app(
            canvas,
            &frame_layout,
            &mut clickable_regions,
            last_frame_dur,
            path_amount,
        );
        process_input(
            &mut run,
            talos.poll_input().expect("Failed to poll input"),
            &env,
            frame_layout,
            &clickable_regions,
            &mut state,
        );

        (last_frame, last_frame_dur) = fps_sleeper(last_frame);
    }
    Ok(())
}
