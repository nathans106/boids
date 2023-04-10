#![feature(allocator_api)]
#![feature(once_cell)]

use iced::{Application, Settings};

mod app;
mod boid;
mod config;
mod simulation;

use crate::app::App;

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
