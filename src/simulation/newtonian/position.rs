use std::{ops::{Add, AddAssign, Sub}};

use pyo3::prelude::*;

use super::{Distance, Vector};

#[pyclass]
#[derive(Clone)]
pub struct Position {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32
}

impl Vector for Position {
    fn at(x: f32, y: f32) -> Self {
        Position { x: x, y: y }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Add<&Distance> for &Position {
    fn add(self, distance: &Distance) -> Position {
        return Position{x: self.x + distance.x(), y: self.y + distance.y()};
    }

    type Output = Position;
}

impl AddAssign<&Distance> for Position {
    fn add_assign(&mut self, distance: &Distance) {
        self.x += distance.x();
        self.y += distance.y();
    }
}

impl Sub<&Position> for &Position {
    type Output = Distance;

    fn sub(self, rhs: &Position) -> Self::Output {
        Distance{dx: self.x - rhs.x, dy: self.y - rhs.y}
    }
}
