use crate::{boid::Boid, newtonian::Force};

pub trait Calculator {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Force;
}
