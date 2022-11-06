use crate::{newtonian::Velocity, boid::Boid};

pub trait Calculator {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity;
}
