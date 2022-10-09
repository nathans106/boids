use serde::Deserialize;

use crate::model::{Boid, Velocity};

use super::Calculator;

#[derive(Deserialize)]
pub struct MatchVelocity {
    rate: f32
}

impl Calculator for MatchVelocity {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let velocities_sum: Velocity = other_boids.iter().map(|other_boid| other_boid.velocity()).sum();
        let num_other = other_boids.len() as f32;
        let velocity_mean = &velocities_sum / &num_other;
        return &(&velocity_mean - boid.velocity()) / &self.rate;
    }
}
