use serde::Deserialize;

use crate::{
    boid::Boid,
    newtonian::{Force, Position, Vector},
};

use super::Calculator;

#[derive(Clone, Deserialize)]
pub struct FlockToCentre {
    time: f32,
}

impl Calculator for FlockToCentre {
    fn calculate(&self, boid: &Boid, other_boids: &[&Boid]) -> Force {
        let other_positions: Vec<&Vector> = other_boids
            .iter()
            .map(|other_boid| other_boid.pos())
            .collect();
        let flock_centre = Position::mean(&other_positions);
        let force = &(&flock_centre - boid.pos()) / &self.time;
        force
    }
}
