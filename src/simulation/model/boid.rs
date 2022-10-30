use std::time::Duration;

use crate::{model::{Position}, velocity_calculator::VelocityCalculator, parameters::Parameters};

use super::Velocity;

#[derive(Clone)]
pub struct Boid {
    pos: Position,
    velocity: Velocity,
    velocity_calculator: VelocityCalculator
}

impl Boid {
    pub fn new(pos: Position, velocity: Velocity, parameters: &Parameters) -> Self {
        Boid{pos: pos, velocity: velocity, velocity_calculator: VelocityCalculator::new(parameters)}
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn velocity(&self) -> &Velocity {
        &self.velocity
    }

    pub fn update(&mut self, other_boids: &[&Boid]) {
        self.velocity = self.velocity_calculator.velocity(&self, other_boids);
    }

    pub fn advance(&mut self, time: &Duration) {
        let distance = &self.velocity * time;
        self.pos += &distance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_() {
        let mut boid = Boid::new(Position::origin(), Velocity::new());
        boid.move_(&Velocity { x: 1.0, x: 1.0 });
        let pos = boid.pos();
        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 1);
    }
}