use std::ops::Div;

use super::{Vector, Mass, Acceleration};

pub struct Force {
    x: f32,
    y: f32
}

impl Vector for Force {
    fn at(x: f32, y: f32) -> Self {
        Force{x: x, y: y}
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Div<&Mass> for &Force {
    type Output = Acceleration;

    fn div(self, mass: &Mass) -> Self::Output {
        Acceleration::at(self.x / mass, self.y / mass)
    }
}

impl Div<&f32> for &Force {
    type Output = Force;

    fn div(self, rhs: &f32) -> Self::Output {
        Force::at(self.x / rhs, self.y / rhs)
    }
}
