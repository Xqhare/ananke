use std::{collections::BTreeMap, path::PathBuf};

use anansi::List;
use areia::BaseDirs;
use brigid::{Brigid, content::Content};
use nabu::{Array, Object, XffValue, xff};
use talos::{
    Talos,
    atlases::StyleAtlas,
    layout::Layout,
    render::{Colour, Extended, Style, TrueColour},
    widgets::stateful::States,
};

use crate::{
    error::{AnankeError, AnankeResult},
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
}

pub struct DiskEnvironment {
    pub brigid: Brigid,
}

impl DiskEnvironment {
    pub fn new() -> AnankeResult<DiskEnvironment> {
        Ok(DiskEnvironment {
            brigid: setup_env()?,
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
                            (
                                List::load(path).map_err(|e| Into::<AnankeError>::into(e))?,
                                ary.len(),
                            )
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
    let states = make_state(path_amount, &list, talos.codex());
    let env = Environment {
        run: true,
        list,
        disk_env,
        styles,
        path_amount,
        gen_layout,
        states,
    };

    Ok((env, talos))
}

fn style_atlas() -> StyleAtlas {
    let default = Style::builder()
        .set_bg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
            30, 25, 20,
        ))))
        .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
            200, 170, 160,
        ))));
    let mut atlas = StyleAtlas::new(Some(default.build()));
    atlas.update_ok(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                0, 255, 0,
            ))))
            .build(),
    );
    atlas.update_warning(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                255, 255, 0,
            ))))
            .build(),
    );
    atlas.update_error(
        default
            .set_fg(Colour::Extended(Extended::TrueColour(TrueColour::RGB(
                255, 0, 0,
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

fn setup_env() -> AnankeResult<Brigid> {
    let root = BaseDirs::new()
        .map_err(|e| Into::<AnankeError>::into(e))?
        .config_local_dir()
        .join("Ananke");

    Brigid::new(&root)
        .file("config.xff", |file| {
            file.with_default_content(Content::XFF(default_config(&root)))
                .with_fallback();
        })
        .add_license(include_str!("../LICENSE"), root.join("LICENSE.txt"))
        .establish()
        .map_err(|e| e.into())
}
