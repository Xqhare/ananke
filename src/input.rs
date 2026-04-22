use std::collections::BTreeMap;

use talos::{
    Talos,
    atlases::LayoutAtlas,
    input::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    widgets::stateful::States,
};

use crate::startup::Environment;

pub fn process_input(
    events: Option<&[Event]>,
    env: &mut Environment,
    clickable_regions: &BTreeMap<String, Rect>,
) {
    if let Some(events) = events {
        for event in events.iter() {
            match event {
                Event::KeyEvent(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers:
                        KeyModifiers {
                            none: true,
                            shift: false,
                            ctrl: false,
                            alt: false,
                        },
                }) => {
                    env.run = false;
                }
                _ => {}
            }
        }
    }
}
