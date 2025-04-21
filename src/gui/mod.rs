use anansi::List;
use eframe::{egui::Vec2, run_native, App, NativeOptions};

use crate::{error::AnankeError, state::{startup_state::StartupState, State}, util::NewTask};

const INITIAL_WINDOW_SIZE: Vec2 = Vec2::new(1000.0, 600.0);

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

impl Ananke {
    pub fn save_todo(&self) -> Result<(), AnankeError> {
        match self.entire_list.save() {
            Ok(()) => Ok(()),
            Err(e) => Err(AnankeError { title: "IO Error".to_string(), message: "Unable to save todo list".to_string(), context: Some(e.to_string()) }),
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
    
    let entire_list = List::new(state.persistent_state.todo_file_path.clone());
    // Only Display Open Tasks ordered by Priority at startup
    let mut display_list: List = entire_list.clone().sort(anansi::SortBy::Priority).into();
    display_list = display_list.open().into();

    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Some(INITIAL_WINDOW_SIZE);
    
    run_native(&app_name, native_options, Box::new(|_| {
        Ok(Box::<Ananke>::new(Ananke { entire_list, first_run: startup_state.first_run, load_file: false , display_list, new_task: NewTask::new(state.persistent_state.timezone), state }))
    })).unwrap()
}
