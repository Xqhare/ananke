use std::collections::BTreeMap;

use talos::{Talos, atlases::LayoutAtlas, input::Event, layout::Rect, widgets::stateful::States};

use crate::startup::Environment;

pub fn process_input(
    events: Option<&[Event]>,
    env: &mut Environment,
    clickable_regions: &BTreeMap<String, Rect>,
) {
}
