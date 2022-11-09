use std::{ops::{Mul, AddAssign, Div, Sub}, time::Duration, iter::Sum};

use super::{Distance, Vector};

#[derive(Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Vector for Velocity {
    fn at(x: f32, y: f32) -> Self {
        Velocity { x: x, y: y }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl AddAssign<&Velocity> for Velocity {
    fn add_assign(&mut self, rhs: &Velocity) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<&f32> for &Velocity {
    type Output = Velocity;

    fn div(self, rhs: &f32) -> Self::Output {
        return self * &(1.0 / rhs);
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

impl Sub<&Velocity> for &Velocity {
    type Output = Velocity;

    fn sub(self, rhs: &Velocity) -> Self::Output {
        return Velocity{ x: self.x - rhs.x, y: self.y - rhs.y };
    }
}

impl<'a> Sum<&'a Velocity> for Velocity {

    fn sum<I: Iterator<Item = &'a Velocity>>(iter: I) -> Self {
        let mut result = Velocity::at(0.0, 0.0);
        for velocity in iter {
            result += velocity;
        }

        return result;
    }
}