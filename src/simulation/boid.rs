use std::time::Duration;

use crate::{newtonian::{Position, Velocity, Mass}, force_calculator::ForceCalculator, parameters::Parameters};

#[derive(Clone)]
pub struct Boid {
    mass: Mass,
    pos: Position,
    velocity: Velocity,
    force_calculator: ForceCalculator
}

impl Boid {
    pub fn new(pos: Position, velocity: Velocity, parameters: &Parameters) -> Self {
        Boid{pos: pos, velocity: velocity, mass: parameters.boid.mass, force_calculator: ForceCalculator::new(parameters)}
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn velocity(&self) -> &Velocity {
        &self.velocity
    }

    pub fn update(&mut self, other_boids: &[&Boid]) {
        let force = self.force_calculator.force(&self, other_boids);
        let acceleration = &force / &self.mass;
        self.velocity += &(&acceleration * &Duration::from_secs(1));
    }

    pub fn advance(&mut self, time: &Duration) {
        let distance = &self.velocity * time;
        self.pos += &distance;
    }
}
