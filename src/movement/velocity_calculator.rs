use std::{collections::HashMap, time::Duration};

use crate::{database::Id, model::{Boid, Velocity, Position}};

pub struct VelocityCalculator {
    pub fly_to_centre_time: Duration
}

impl VelocityCalculator {
    pub fn new() -> Self {
        VelocityCalculator { fly_to_centre_time: Duration::from_secs(100) }
    }

    pub fn velocities(&self, boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
        let mut velocities: HashMap<Id, Velocity> = HashMap::new();

        for (id, boid) in boids {
            let other_boids = boids.iter().filter(|(other_id, _other_boid)| other_id != &id);
            let other_positions: Vec<&Position> = other_boids.map(|(_other_id, other_boid)| other_boid.pos()).collect();
            
            let flock_centre = Position::centre(other_positions.into_iter());
            let velocity = &(&flock_centre - boid.pos()) / &self.fly_to_centre_time;
            velocities.insert(id.clone(), velocity);
        }

        return velocities;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velocities() {
        let mut boids = HashMap::new();
        boids.insert(2, Boid::at(Position::new(0.0, 0.0)));

        let velocity_calculator = VelocityCalculator::new();
        let result = velocity_calculator.velocities(&boids);
        for velocity in result.values() {
            assert_eq!(velocity.x, 1.0);
            assert_eq!(velocity.x, 1.0);
        }
    }
}
