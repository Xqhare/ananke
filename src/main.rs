use std::{collections::BTreeMap, time::Instant};

use talos::layout::Rect;

use crate::{
    error::{AnankeError, AnankeResult},
    input::{Focus, process_input},
    layout::make_frame_layout,
    render::render_app,
    startup::startup,
    utils::fps_sleeper,
};

mod error;
mod input;
mod layout;
mod render;
mod startup;
mod state;
mod utils;

fn main() -> AnankeResult<()> {
    // Create the environment, state, and determine the state and whether this is the first run
    let (mut env, mut talos) = startup()?;

    // The clickable regions of the current frame
    let mut clickable_regions: BTreeMap<String, Rect> = BTreeMap::new();

    // Metadata of the last frame
    let mut last_frame = Instant::now();
    let mut last_frame_dur = 0;

    // This is for the borrow checker complaining about mutability
    let codex = &talos.codex().clone();

    // This is needed for persistent focus of entry fields inside the input processing
    let mut focus = Focus::None;
    // Render loop
    while env.run {
        // Reset the canvas
        talos.begin_frame();

        // Construct frame dependent layout
        let (canvas, _) = talos.render_ctx();
        let frame_layout = make_frame_layout(&canvas.size_rect(), &env.gen_layout);

        // Render the app
        render_app(
            canvas,
            codex,
            &frame_layout,
            &mut clickable_regions,
            last_frame_dur,
            &mut env,
        );

        // Process input (Both clicks and key events)
        // This is also the place where state mutations happen
        if let Some(f) = process_input(
            codex,
            talos
                .poll_input()
                .map_err(|err| Into::<AnankeError>::into(err))?,
            &mut env,
            &clickable_regions,
            &focus,
        ) {
            focus = f;
        }

        // Actual render of the canvas to the Terminal
        talos
            .present()
            .map_err(|err| Into::<AnankeError>::into(err))?;

        // Sleep until next frame, if needed & cap to specified fps
        (last_frame, last_frame_dur) = fps_sleeper(last_frame);
    }
    // Save the list before exiting
    let _ = env.list.save();
    // Talos cleanup is automatic when dropping talos
    Ok(())
}
