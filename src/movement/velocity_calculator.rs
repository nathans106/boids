use std::{collections::HashMap, time::Duration};

use crate::{database::Id, model::{Boid, Velocity, Position, Distance}};

pub struct VelocityCalculator {
    pub fly_to_centre_time: Duration,
    pub avoidance_distance: f32,
    pub avoidance_time: Duration
}

impl VelocityCalculator {
    pub fn new() -> Self {
        VelocityCalculator {
            fly_to_centre_time: Duration::from_secs(100),
            avoidance_distance: 1000.0,
            avoidance_time: Duration::from_secs(300)
        }
    }

    pub fn velocities(&self, boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
        let mut velocities: HashMap<Id, Velocity> = HashMap::new();

        for (id, boid) in boids {
            let other_boids = boids.iter().filter(|(other_id, _other_boid)| other_id != &id);
            let other_positions: Vec<Position> = other_boids.map(|(_other_id, other_boid)| other_boid.pos().clone()).collect();

            let mut velocity = Velocity::new();
            velocity += &self.flock_to_centre_velocity(boid, &other_positions);
            velocity += &self.avoid_collision_velocity(boid, &other_positions);

            velocities.insert(id.clone(), self.flock_to_centre_velocity(boid, &other_positions));
        }

        return velocities;
    }

    fn flock_to_centre_velocity(&self, boid: &Boid, other_positions: &Vec<Position>) -> Velocity {
        let flock_centre = Position::centre(other_positions.iter());
        let velocity = &(&flock_centre - boid.pos()) / &self.fly_to_centre_time;
        return velocity;
    }

    fn avoid_collision_velocity(&self, boid: &Boid, other_positions: &Vec<Position>) -> Velocity {
        let too_close = other_positions.iter().filter(|other_pos| 
        (boid.pos() - other_pos).absolute() <= self.avoidance_distance
        );
        let distances = too_close.map(|other_pos| boid.pos() - other_pos);
        let sum: Distance = distances.sum();
        let velocity = &sum / &self.avoidance_time;
        return &velocity * &-1.0;
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
