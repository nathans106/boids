use std::{ops::{Div, AddAssign}, time::Duration, iter::Sum};

use super::{Velocity, Vector};


#[derive(Clone)]
pub struct Distance {
    dx: f32,
    dy: f32
}

impl Vector for Distance {
    fn at(x: f32, y: f32) -> Self {
        Distance{dx: 0.0, dy: 0.0}
    }

    fn x(&self) -> f32 {
        self.dx
    }

    fn y(&self) -> f32 {
        self.dy
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, rhs: Self) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
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

impl Sum for Distance {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Distance::new();
        for distance in iter {
            result += distance;
        }

        return result;
    }
}
