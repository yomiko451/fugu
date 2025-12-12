use crate::{app::App, common::*};

mod common;
mod editor;
mod preview;
mod file_panel;
mod setting;
mod status_bar;
mod menu_bar;
mod app;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(APP_NAME)
        .window_size(DEFAULT_WINDOW_SIZE)
        .theme(DEFAULT_THEME.clone())
        .run()
}
