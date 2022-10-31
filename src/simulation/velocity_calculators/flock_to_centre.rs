use std::time::Duration;

use serde::Deserialize;

use crate::{newtonian::{Velocity, Position}, boid::Boid};

use super::Calculator;

#[derive(Clone, Deserialize)]
pub struct FlockToCentre {
    time: u64
}

impl Calculator for FlockToCentre {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let other_positions = other_boids.iter().map(|other_boid| other_boid.pos());
        let flock_centre = Position::centre(other_positions);
        let fly_to_centre_time = Duration::from_secs(self.time);
        let velocity = &(&flock_centre - boid.pos()) / &fly_to_centre_time;
        return velocity;
    }
}
