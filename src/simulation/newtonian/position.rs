use std::{ops::{Add, AddAssign, Sub}};

use pyo3::prelude::*;

use super::Distance;

#[pyclass]
#[derive(Clone)]
pub struct Position {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position{x: x, y: y}
    }
}

impl Vector for Position {
    
}

impl Add<&Distance> for &Position {
    fn add(self, distance: &Distance) -> Position {
        return Position{x: self.x + distance.dx, y: self.y + distance.dy};
    }

    type Output = Position;
}

impl AddAssign<&Distance> for Position {
    fn add_assign(&mut self, distance: &Distance) {
        self.x += distance.dx;
        self.y += distance.dy;
    }
}

impl Sub<&Position> for &Position {
    type Output = Distance;

    fn sub(self, rhs: &Position) -> Self::Output {
        Distance{dx: self.x - rhs.x, dy: self.y - rhs.y}
    }
}
