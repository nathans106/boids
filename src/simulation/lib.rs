#![feature(array_zip)]
#![allow(clippy::borrow_deref_ref)]

mod boid;
mod database;
mod newtonian;
mod simulation;
pub use simulation::Simulation;
mod force_calculator;
mod force_calculators;
mod parameters;
mod two_d;
