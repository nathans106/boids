use std::{ops::{Add, AddAssign, Sub, Div}};

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Pos {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos{x: x, y: y}
    }

    pub fn origin() -> Self {
        Pos{x: 0.0, y: 0.0}
    }

    pub fn centre<'a, I>(positions: I) -> Pos
        where I: Iterator<Item = &'a Pos>
    {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for pos in positions {
            count += 1;
            sum_x += pos.x;
            sum_y += pos.y;
        }

        return Pos{x: sum_x / count as f32, y: sum_y / count as f32};
    }
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

impl Sub<&Pos> for Pos {
    type Output = Velocity;

    fn sub(self, rhs: &Pos) -> Self::Output {
        Velocity{dx: self.x - rhs.x, dy: self.y - rhs.y}
    }
}

#[derive(Clone)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32
}

impl Div<&f32> for Velocity {
    type Output = Velocity;

    fn div(self, rhs: &f32) -> Self::Output {
        Velocity{ dx: self.dx / rhs, dy: self.dy / rhs}
    }
}