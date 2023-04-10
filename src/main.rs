#![feature(allocator_api)]

use iced::{Application, Settings};

mod app;
mod boid;
mod simulation;

use crate::app::App;

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
