use std::collections::BTreeMap;

use talos::{Talos, atlases::LayoutAtlas, input::Event, layout::Rect, widgets::stateful::States};

use crate::startup::State;

pub fn process_input(
    run: &mut bool,
    events: Option<&[Event]>,
    env: &State,
    frame_layout: LayoutAtlas,
    clickable_regions: &BTreeMap<String, Rect>,
    state: &mut BTreeMap<String, States>,
) {
    todo!()
}
