use crate::{newtonian::Force, boid::Boid};

pub trait Calculator {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Force;
}
