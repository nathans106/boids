use std::{ops::Mul, time::Duration};

use super::{Vector, Velocity};

pub struct Acceleration {
    x: f32,
    y: f32
}

impl Vector for Acceleration {
    fn at(x: f32, y: f32) -> Self {
        Acceleration { x: x, y: y }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Mul<&Duration> for &Acceleration {
    type Output = Velocity;

    fn mul(self, duration: &Duration) -> Self::Output {
        let seconds = duration.as_secs_f32();
        Velocity{x: self.x * seconds, y: self.y * seconds}
    }
}
