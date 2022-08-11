use std::{ops::Mul, time::Duration};

use super::{Distance};

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Mul<&Duration> for &Velocity {
    type Output = Distance;

    fn mul(self, duration: &Duration) -> Self::Output {
        let seconds = duration.as_secs_f32();
        return Distance { dx: self.x * seconds, dy: self.y * seconds}
    }
}
