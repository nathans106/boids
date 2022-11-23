use crate::{
    force_calculator::ForceCalculator,
    newtonian::{Acceleration, Mass, Position, Scalar, Vector, Velocity},
    parameters::Parameters,
    simulation::TICK,
};

#[derive(Clone)]
pub struct Boid {
    mass: Mass,
    pos: Position,
    velocity: Velocity,
    acceleration: Acceleration,
    force_calculator: ForceCalculator,
}

impl Boid {
    pub fn new(pos: Position, velocity: Velocity, parameters: &Parameters) -> Self {
        Boid {
            pos: pos,
            velocity: velocity,
            acceleration: Vector::origin(),
            mass: parameters.boid.mass,
            force_calculator: ForceCalculator::new(parameters),
        }
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn velocity(&self) -> &Velocity {
        &self.velocity
    }

    pub fn update(&mut self, other_boids: &[&Boid]) {
        let force = self.force_calculator.force(&self, other_boids);
        self.acceleration = &force / &self.mass;
    }

    pub fn advance(&mut self, time: &Scalar) {
        self.velocity += &(&self.acceleration * &TICK);
        let distance = &self.velocity * time;
        self.pos += &distance;
    }
}
