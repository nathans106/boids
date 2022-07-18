use std::ops::{Add, AddAssign};

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Pos {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32
}

impl Add<&Velocity> for Pos {
    fn add(mut self, velocity: &Velocity) -> Self {
        self.x += velocity.dx;
        self.y += velocity.dy;
        return self;
    }

    type Output = Self;
}

impl AddAssign<&Velocity> for Pos {
    fn add_assign(&mut self, velocity: &Velocity) {
        self.x += velocity.dx;
        self.y += velocity.dy;
    }
}

#[derive(Clone)]
pub struct Velocity {
    pub dx: i32,
    pub dy: i32
}
