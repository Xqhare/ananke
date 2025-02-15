use anansi::List;
use eframe::{run_native, App, NativeOptions};

use crate::state::{startup_state::StartupState, State};

mod main_screen;
mod menu_bar;

pub struct Ananke {
    pub first_run: bool,
    pub state: State,
    pub entire_list: List,
}

impl App for Ananke {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.first_run {
            // TODO: Add a welcome screen
            self.first_run = false;
        } else {
            self.menu_bar(ctx, frame);
            self.main_screen(ctx, frame);
        }
    }
}

fn get_app_name() -> String {
    let mut app_name = env!("CARGO_PKG_NAME").to_string();
    let letter = app_name.remove(0);
    app_name.insert(0, letter.to_ascii_uppercase());
    app_name
}

pub fn gui_startup(startup_state: StartupState) {
    let app_name = get_app_name();
    let state = State::new(startup_state.persistent_state);
    let native_options = NativeOptions::default();
    run_native(&app_name, native_options, Box::new(|_| {
        Ok(Box::<Ananke>::new(Ananke { entire_list: List::new(state.persistent_state.file_path.clone()), first_run: startup_state.first_run, state }))
    })).unwrap()
}
