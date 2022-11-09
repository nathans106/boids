use std::time::Duration;

use serde::Deserialize;

use crate::{newtonian::{Force, Distance, Vector}, boid::Boid};

use super::Calculator;

#[derive(Clone, Deserialize)]
pub struct AvoidCollision {
    distance: f32,
    intensity: Distance
}

impl Calculator for AvoidCollision {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Force {
        let other_positions = other_boids.iter().map(|other_boid| other_boid.pos());
        let too_close = other_positions.filter(|other_pos| 
        (boid.pos() - other_pos).abs() <= self.distance
        );
        let distances = too_close.map(|other_pos| boid.pos() - other_pos);
        let sum: Distance = distances.sum();
        return (&sum / &self.intensity) * Force::unit();
    }
}
