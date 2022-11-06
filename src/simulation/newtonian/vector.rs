pub trait Vector {
    fn at(x: f32, y: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn origin() -> Self where Self: Sized {
        Self::at(0.0, 0.0)
    }

    fn abs(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }

    fn mean(vectors: &[&Self]) -> Self where Self: Sized {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for vector in vectors {
            count += 1;
            sum_x += vector.x();
            sum_y += vector.y();
        }

        return Self::at(sum_x / count as f32, sum_y / count as f32);
    }
}
