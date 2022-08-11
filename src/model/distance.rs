use std::{ops::Div, time::Duration};

use super::{Velocity};


#[derive(Clone)]
pub struct Distance {
    pub dx: f32,
    pub dy: f32
}

impl Div<&f32> for &Distance {
    type Output = Distance;

    fn div(self, rhs: &f32) -> Self::Output {
        Distance{ dx: self.dx / rhs, dy: self.dy / rhs}
    }
}

impl Div<&Duration> for &Distance {
    type Output = Velocity;

    fn div(self, duration: &Duration) -> Self::Output {
        let seconds = duration.as_secs_f32();
        Velocity{ x: self.dx / seconds, y: self.dy / seconds }
    }
}
