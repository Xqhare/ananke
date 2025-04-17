use anansi::List;
use eframe::{egui::Vec2, run_native, App, NativeOptions};

use crate::{state::{startup_state::StartupState, State}, util::NewTask};

mod main_screen;
mod menu_bar;
mod error;

pub struct Ananke {
    first_run: bool,
    load_file: bool,
    state: State,
    entire_list: List,
    display_list: List,
    new_task: NewTask,
}

impl App for Ananke {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.state.error.is_some() {
            self.error_screen(ctx);
        } else {
            if self.first_run {
                // TODO: Add a welcome screen
                self.first_run = false;
            } else if self.load_file {
                self.menu_bar(ctx, frame);
                self.load_file(ctx, frame);
            } else {
                self.menu_bar(ctx, frame);
                self.main_screen(ctx, frame);
            }
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
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Some(Vec2::new(900.0, 600.0));
    let list = List::new(state.persistent_state.todo_file_path.clone());
    run_native(&app_name, native_options, Box::new(|_| {
        Ok(Box::<Ananke>::new(Ananke { entire_list: list.clone(), first_run: startup_state.first_run, load_file: false , display_list: list, new_task: NewTask::new(state.persistent_state.timezone), state }))
    })).unwrap()
}
