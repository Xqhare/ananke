//! # Startup & Environment Orchestration
//!
//! This module handles the initialization of the application environment, demonstrating how 
//! the **Pantheon** stack manages persistent data and configuration.
//!
//! - **Areia**: Handles platform-specific base directory discovery.
//! - **Brigid**: Manages the application's configuration directory, file integrity, and fallback logic.
//! - **Nabu**: Handles the **XFF v3** (Xqhare File Format) serialization and data modeling.
//! - **Anansi**: Provides the domain logic for the `todo.txt` format.

use std::path::PathBuf;

use anansi::{List, Task};
use areia::BaseDirs;
use brigid::{Brigid, content::Content};
use nabu::{Object, XffValue, xff};
use talos::{
    Talos,
    atlases::StyleAtlas,
    layout::Layout,
    render::{Colour, Extended, Style, TrueColour},
};

use crate::{
    error::{AnankeError, AnankeResult},
    keys::styles::{BLUE, CURSOR, DEFAULT_INVERTED, EDITABLE_ACTIVE, EDITABLE_INACTIVE},
    layout::make_layout,
    state::{UiState, make_state},
};

/// The unified runtime environment for Ananke.
pub struct Environment {
    /// The current todo list (managed by Anansi).
    pub list: List,
    /// Persistent disk environment (managed by Brigid).
    pub disk_env: DiskEnvironment,
    /// Centralized style definitions for the UI.
    pub styles: StyleAtlas,
    /// Number of distinct todo lists tracked in config.
    pub path_amount: usize,
    /// The root layout definition.
    pub gen_layout: Layout,
    /// The active UI state (TextBoxes, Table state, etc.).
    pub ui_state: UiState,
    /// Control flag for the main loop.
    pub run: bool,
    /// Scratch task for the 'Creator' UI.
    pub new_task: Task,
    /// The subset of tasks currently being rendered.
    pub render_tasks: Vec<Task>,
}

/// Manages the physical files on disk using Brigid.
pub struct DiskEnvironment {
    /// The Brigid instance for file operations.
    pub brigid: Brigid,
    /// The user's home directory path (from Areia).
    pub home_path: PathBuf,
}

impl DiskEnvironment {
    /// Initializes the disk environment, creating config folders if they don't exist.
    pub fn new() -> AnankeResult<DiskEnvironment> {
        let (brigid, home_path) = setup_env()?;
        Ok(DiskEnvironment { brigid, home_path })
    }
}

/// The core startup sequence.
///
/// It follows these steps:
/// 1. Locates/Creates config files using Brigid.
/// 2. Loads the todo list path from `config.xff` (Nabu).
/// 3. Initializes the Talos rendering engine.
/// 4. Builds the initial UI state.
pub fn startup() -> AnankeResult<(Environment, Talos)> {
    let disk_env = DiskEnvironment::new()?;
    
    // Load the configuration from disk.
    // Brigid ensures that `config.xff` exists; if missing, it uses the default provided in `setup_env`.
    let (list, path_amount) =
        if let Some(conf) = disk_env.brigid.get_file("config.xff")?.into_object() {
            if let Some(paths) = conf.get("paths") {
                if let Some(ary) = paths.as_array() {
                    if ary.len() == 0 {
                        Err(AnankeError::Startup("Paths array is empty".to_string()))?
                    } else {
                        // Extract the most recent path from the XFF array.
                        let path = &ary[ary.len().saturating_sub(1)];
                        if let Some(path) = path.as_string() {
                            if PathBuf::from(path).exists() {
                                (
                                    List::load(path).map_err(|e| Into::<AnankeError>::into(e))?,
                                    ary.len(),
                                )
                            } else {
                                // Fallback: Create a new list if the file doesn't exist yet.
                                (List::new(path), ary.len())
                            }
                        } else {
                            Err(AnankeError::Startup(
                                "Paths array first element is not a string".to_string(),
                            ))?
                        }
                    }
                } else {
                    Err(AnankeError::Startup("Paths is not an array".to_string()))?
                }
            } else {
                Err(AnankeError::Startup("Missing paths".to_string()))?
            }
        } else {
            Err(AnankeError::Startup("Missing config.xff".to_string()))?
        };

    let styles = style_atlas();
    let talos = Talos::builder()
        .build()
        .map_err(|e| Into::<AnankeError>::into(e))?;
    let gen_layout = make_layout();
    let ui_state = make_state(path_amount, &list, talos.codex(), &disk_env.home_path);
    let new_task = Task::new("", list.max_id());
    let render_tasks = list.tasks();

    let env = Environment {
        run: true,
        list,
        disk_env,
        styles,
        path_amount,
        gen_layout,
        ui_state,
        new_task,
        render_tasks,
    };

    Ok((env, talos))
}

/// Defines the global color scheme and UI styles.
///
/// This demonstrates Talos's support for **TrueColour** (24-bit) rendering,
/// bypassing the limitations of traditional 8-bit terminal colors.
fn style_atlas() -> StyleAtlas {
    let default_bg = Colour::Extended(Extended::TrueColour(TrueColour::RGB(
        12, 11, 10, // Deep obsidian-brown
    )));
    let default_fg = Colour::Extended(Extended::TrueColour(TrueColour::RGB(
        255, 220, 195, // Soft cream
    )));
    
    let default = Style::builder().set_bg(default_bg).set_fg(default_fg);
    let mut atlas = StyleAtlas::new(Some(default.build()));

    // Define semantic styles (Ok, Warning, Error)
    atlas.update_ok(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(0, 255, 0))))
            .build(),
    );
    atlas.update_warning(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                255, 255, 0, // Yellow
            ))))
            .build(),
    );
    atlas.update_error(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                255, 0, 0, // Red
            ))))
            .build(),
    );
    atlas.insert(
        DEFAULT_INVERTED.to_string(),
        default.set_fg(default_bg).set_bg(default_fg).build(),
    );
    atlas.insert(
        CURSOR.to_string(),
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                255, 255, 255,
            ))))
            .set_bg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                0, 0, 0,
            ))))
            .set_blink(true)
            .build(),
    );
    atlas.insert(
        EDITABLE_ACTIVE.to_string(),
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                255, 255, 255, // White
            ))))
            .build(),
    );
    atlas.insert(
        EDITABLE_INACTIVE.to_string(),
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                100, 100, 100, // Grey
            ))))
            .build(),
    );
    atlas.insert(
        BLUE.to_string(),
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                0, 150, 205, // Blue
            ))))
            .build(),
    );
    atlas
}

/// Generates the default XFF configuration for new installations.
fn default_config(root: &PathBuf) -> XffValue {
    let mut obj = Object::new();
    obj.insert(
        "paths",
        xff!(vec![xff!(
            root.join("default-list.txt").display().to_string()
        )]),
    );
    xff!(obj)
}

/// Bootstraps the Brigid environment.
///
/// It uses Areia to find platform-specific directories (e.g., `~/.config/` on Linux)
/// and then initializes Brigid to manage the 'Ananke' subdirectory.
fn setup_env() -> AnankeResult<(Brigid, PathBuf)> {
    let dirs = BaseDirs::new().map_err(|e| Into::<AnankeError>::into(e))?;
    let root = dirs.config_local_dir().join("Ananke");
    let home = dirs.home_dir();

    Ok((
        Brigid::new(&root)
            .file("config.xff", |file| {
                file.with_default_content(Content::XFF(default_config(&root)))
                    .with_fallback();
            })
            // Brigid can also manage license files and other static assets.
            .add_license(include_str!("../LICENSE"), root.join("LICENSE.txt"))
            .establish()
            .map_err(|e| Into::<AnankeError>::into(e))?,
        home.to_path_buf(),
    ))
}
