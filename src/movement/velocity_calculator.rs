use std::{collections::HashMap, time::Duration};

use crate::{database::Id, model::{Boid, Velocity, Position, Distance}};

pub struct VelocityCalculator {
    pub fly_to_centre_time: Duration,
    pub avoidance_distance: f32,
    pub avoidance_time: Duration,
    pub velocity_match_rate: f32
}

impl VelocityCalculator {
    pub fn new() -> Self {
        VelocityCalculator {
            fly_to_centre_time: Duration::from_secs(100),
            avoidance_distance: 40.0,
            avoidance_time: Duration::from_secs(50),
            velocity_match_rate: 3.0
        }
    }

    pub fn velocities(&self, boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
        let mut velocities: HashMap<Id, Velocity> = HashMap::new();

        for (id, boid) in boids {
            let other_boids: Vec<&Boid> = boids.iter().filter(|(other_id, _other_boid)| other_id != &id).map(|(_other_id, other_boid)|other_boid).collect();

            let mut velocity = Velocity::new();
            velocity += &self.flock_to_centre_velocity(boid, &other_boids);
            velocity += &self.avoid_collision_velocity(boid, &other_boids);
            velocity += &self.match_velocity_velocity(boid, &other_boids);

            velocities.insert(id.clone(), velocity);
        }

        return velocities;
    }

    fn flock_to_centre_velocity(&self, boid: &Boid, other_boids: &Vec<&Boid>) -> Velocity {
        let other_positions = other_boids.iter().map(|other_boid| other_boid.pos());
        let flock_centre = Position::centre(other_positions);
        let velocity = &(&flock_centre - boid.pos()) / &self.fly_to_centre_time;
        return velocity;
    }

    fn avoid_collision_velocity(&self, boid: &Boid, other_boids: &Vec<&Boid>) -> Velocity {
        let other_positions = other_boids.iter().map(|other_boid| other_boid.pos());
        let too_close = other_positions.filter(|other_pos| 
        (boid.pos() - other_pos).absolute() <= self.avoidance_distance
        );
        let distances = too_close.map(|other_pos| boid.pos() - other_pos);
        let sum: Distance = distances.sum();
        return &sum / &self.avoidance_time;
    }

    fn match_velocity_velocity(&self, boid: &Boid, other_boids: &Vec<&Boid>) -> Velocity {
        let velocities_sum: Velocity = other_boids.iter().map(|other_boid| other_boid.velocity()).sum();
        let num_other = other_boids.len() as f32;
        let velocity_mean = &velocities_sum / &num_other;
        return &(&velocity_mean - boid.velocity()) / &self.velocity_match_rate;
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
