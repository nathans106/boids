use super::Vector;

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
