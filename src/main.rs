//! # Ananke: The Entry Point
//!
//! This module orchestrates the main application lifecycle, demonstrating the core pattern of the **Talos**
//! rendering engine: a reactive, frame-based loop.
//!
//! The application flow follows a strict **Startup -> Render -> Input -> Present** cycle, ensuring
//! predictable state mutations and high-performance terminal rendering.

use std::{collections::BTreeMap, time::Instant};

use talos::layout::Rect;

use crate::{
    error::{AnankeError, AnankeResult},
    input::process_input,
    layout::make_frame_layout,
    render::render_app,
    startup::startup,
    state::Focus,
    utils::fps_sleeper,
};

mod error;
mod input;
mod keys;
mod layout;
mod render;
mod startup;
mod state;
mod utils;

fn main() -> AnankeResult<()> {
    // --- 1. Startup Phase ---
    // Initialize the environment (using Brigid for filesystem & Anansi for todo logic)
    // and the Talos rendering engine. This phase is handled by `startup.rs`.
    let (mut env, mut talos) = startup()?;

    // Tracks regions of the terminal that are interactive (clickable). 
    // This map is reconstructed every frame during the render phase.
    let mut clickable_regions: BTreeMap<String, Rect> = BTreeMap::new();

    // Timing metadata for frame-rate capping and delta-time calculations.
    let mut last_frame = Instant::now();
    let mut last_frame_dur = 0;

    // The Codex holds the font and glyph information. 
    // Cloning it here (it's an Arc/Reference internally) avoids borrow-checker issues during the loop.
    let codex = &talos.codex().clone();

    // Persistent focus tracking for text entry fields.
    let mut focus = Focus::None;

    // --- 2. Main Render Loop ---
    // The application continues running as long as `env.run` is true.
    while env.run {
        // Signals the start of a new frame to the Talos engine.
        talos.begin_frame();

        // Dynamically construct the layout based on the current terminal size.
        // Talos uses a constraint-based layout system.
        let (canvas, _) = talos.render_ctx();
        let frame_layout = make_frame_layout(&canvas.size_rect(), &env.gen_layout);

        // --- 3. Render Phase ---
        // Draws all UI components to the internal canvas.
        // This also populates the `clickable_regions` map.
        render_app(
            canvas,
            codex,
            &frame_layout,
            &mut clickable_regions,
            last_frame_dur,
            &mut env,
        );

        // --- 4. Input Processing Phase ---
        // Polls the terminal for keyboard and mouse events.
        // State mutations (like adding a task) happen strictly within this block.
        if let Some(foci) = process_input(
            codex,
            talos
                .poll_input()
                .map_err(|err| Into::<AnankeError>::into(err))?,
            &mut env,
            &clickable_regions,
            &focus,
        ) {
            focus = foci;
        }

        // --- 5. Presentation Phase ---
        // Flushes the internal canvas to the actual terminal output.
        talos
            .present()
            .map_err(|err| Into::<AnankeError>::into(err))?;

        // --- 6. Frame Rate Control ---
        // Caps the application to a target FPS (defined in utils) to prevent CPU saturation.
        (last_frame, last_frame_dur) = fps_sleeper(last_frame);
    }

    // --- 7. Cleanup & Persistance ---
    // Ensure the todo list is saved back to disk before exiting.
    // Brigid and Talos cleanup is handled automatically via Drop traits.
    let _ = env.list.save();
    Ok(())
}
