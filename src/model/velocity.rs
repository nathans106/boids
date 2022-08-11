use std::{ops::{Mul, AddAssign}, time::Duration};

use super::{Distance};

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Velocity {
    pub fn new() -> Self {
        Velocity{x: 0.0, y: 0.0}
    }
}

impl AddAssign<&Velocity> for Velocity {
    fn add_assign(&mut self, rhs: &Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<&Duration> for &Velocity {
    type Output = Distance;

    fn mul(self, duration: &Duration) -> Self::Output {
        let seconds = duration.as_secs_f32();
        return Distance { dx: self.x * seconds, dy: self.y * seconds}
    }
}

impl Mul<&f32> for &Velocity {
    type Output = Velocity;

    fn mul(self, rhs: &f32) -> Self::Output {
        Velocity{x: self.x * rhs, y: self.y * rhs}
    }
}
