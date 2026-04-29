use std::{collections::BTreeMap, path::PathBuf};

use anansi::{List, Task};
use areia::BaseDirs;
use brigid::{Brigid, content::Content};
use nabu::{Object, XffValue, xff};
use talos::{
    Talos,
    atlases::StyleAtlas,
    layout::Layout,
    render::{Colour, Extended, Style, TrueColour},
    widgets::stateful::States,
};

use crate::{
    error::{AnankeError, AnankeResult},
    keys::styles::{BLUE, CURSOR, DEFAULT_INVERTED, EDITABLE_ACTIVE, EDITABLE_INACTIVE},
    layout::make_layout,
    state::make_state,
};

pub struct Environment<'a> {
    pub list: List,
    pub disk_env: DiskEnvironment,
    pub styles: StyleAtlas,
    pub path_amount: usize,
    pub gen_layout: Layout,
    pub states: BTreeMap<String, States<'a>>,
    pub run: bool,
    pub new_task: Task,
    pub render_tasks: Vec<Task>,
}

pub struct DiskEnvironment {
    pub brigid: Brigid,
    pub home_path: PathBuf,
}

impl DiskEnvironment {
    pub fn new() -> AnankeResult<DiskEnvironment> {
        let tmp = setup_env()?;
        Ok(DiskEnvironment {
            brigid: tmp.0,
            home_path: tmp.1,
        })
    }
}

/// Startup function
///
/// # Returns
/// - `AnankeResult<(State, Talos, Layout, usize)>`
/// - `State` - The state of the application
/// - `Talos` - The talos instance
/// - `Layout` - The layout of the application
/// - `usize` - The amount of paths
pub fn startup<'a>() -> AnankeResult<(Environment<'a>, Talos)> {
    let disk_env = DiskEnvironment::new()?;
    let (list, path_amount) =
        if let Some(conf) = disk_env.brigid.get_file("config.xff")?.into_object() {
            if let Some(paths) = conf.get("paths") {
                if let Some(ary) = paths.as_array() {
                    if ary.len() == 0 {
                        Err(AnankeError::Startup("Paths array is empty".to_string()))?
                    } else {
                        let path = &ary[ary.len().saturating_sub(1)];
                        if let Some(path) = path.as_string() {
                            if PathBuf::from(path).exists() {
                                (
                                    List::load(path).map_err(|e| Into::<AnankeError>::into(e))?,
                                    ary.len(),
                                )
                            } else {
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
                // Should never happen because of fallback in Brigid
                Err(AnankeError::Startup("Missing paths".to_string()))?
            }
        } else {
            // Should never happen because of fallback in Brigid
            Err(AnankeError::Startup("Missing config.xff".to_string()))?
        };
    let styles = style_atlas();
    let talos = Talos::builder()
        .build()
        .map_err(|e| Into::<AnankeError>::into(e))?;
    let gen_layout = make_layout();
    let states = make_state(path_amount, &list, talos.codex(), &disk_env.home_path);
    let new_task = Task::new("", list.max_id());
    let render_tasks = list.tasks();
    let env = Environment {
        run: true,
        list,
        disk_env,
        styles,
        path_amount,
        gen_layout,
        states,
        new_task,
        render_tasks,
    };

    Ok((env, talos))
}

fn style_atlas() -> StyleAtlas {
    let default_bg = Colour::Extended(Extended::TrueColour(TrueColour::RGB(
        12, 11, 10, // Very Dark Brown
           // 30, 25, 20, // Brown
    )));
    let default_fg = Colour::Extended(Extended::TrueColour(TrueColour::RGB(
        255, 220, 195, // Cream
    )));
    let default = Style::builder().set_bg(default_bg).set_fg(default_fg);
    let mut atlas = StyleAtlas::new(Some(default.build()));
    // Update defaults to use true colour; The default 8-bit colours are not guaranteed to be
    // the colour one expects, if the Terminal Profile changes them.
    atlas.update_ok(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                0, 255, 0, // Green
            ))))
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
            .add_license(include_str!("../LICENSE"), root.join("LICENSE.txt"))
            .establish()
            .map_err(|e| Into::<AnankeError>::into(e))?,
        home.to_path_buf(),
    ))
}
