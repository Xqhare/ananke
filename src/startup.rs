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

pub struct State {
    pub list: List,
    pub env: Environment,
    pub styles: StyleAtlas,
}

pub struct Environment {
    pub brigid: Brigid,
}

impl Environment {
    pub fn new() -> AnankeResult<Environment> {
        Ok(Environment {
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
pub fn startup<'a>() -> AnankeResult<(State, Talos, Layout, usize, BTreeMap<String, States<'a>>)> {
    let env = Environment::new()?;
    let (list, amount) = if let Some(conf) = env.brigid.get_file("config.xff")?.into_object() {
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
    let layout = make_layout();
    let state = State { list, env, styles };
    let widget_state = make_state(&state, amount, talos.codex());

    Ok((state, talos, layout, amount, widget_state))
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
