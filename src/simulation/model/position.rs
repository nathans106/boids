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

    pub fn origin() -> Self {
        Position{x: 0.0, y: 0.0}
    }

    pub fn centre<'a, I>(positions: I) -> Position
        where I: Iterator<Item = &'a Position>
    {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for pos in positions {
            count += 1;
            sum_x += pos.x;
            sum_y += pos.y;
        }

        return Position{x: sum_x / count as f32, y: sum_y / count as f32};
    }
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
