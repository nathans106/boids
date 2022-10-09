use crate::model::{Boid, Velocity};

pub trait Calculator {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity;
}
