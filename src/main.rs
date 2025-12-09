use crate::{app::App, constants::{APP_NAME, DEFAULT_THEME, DEFAULT_WINDOW_SIZE}};

mod constants;
mod editor;
mod preview;
mod file_panel;
mod setting;
mod status_bar;
mod menu_bar;
mod app;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(APP_NAME)
        .window_size(DEFAULT_WINDOW_SIZE)
        .theme(DEFAULT_THEME.clone())
        .run()
}
