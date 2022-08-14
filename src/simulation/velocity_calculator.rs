use std::{time::Duration};

use crate::{model::{Boid, Velocity, Position, Distance}, Parameters};

pub struct VelocityCalculator {
    pub parameters: Parameters
}

impl VelocityCalculator {
    pub fn new() -> Self {
        VelocityCalculator {
            parameters: Parameters::new()
        }
    }

    pub fn velocity(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let mut velocity = Velocity::new();
        velocity += &self.flock_to_centre_velocity(boid, &other_boids);
        velocity += &self.avoid_collision_velocity(boid, &other_boids);
        velocity += &self.match_velocity_velocity(boid, &other_boids);
        return velocity;
    }

    fn flock_to_centre_velocity(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let other_positions = other_boids.iter().map(|other_boid| other_boid.pos());
        let flock_centre = Position::centre(other_positions);
        let fly_to_centre_time = Duration::from_secs(self.parameters.fly_to_centre_time);
        let velocity = &(&flock_centre - boid.pos()) / &fly_to_centre_time;
        return velocity;
    }

    fn avoid_collision_velocity(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let other_positions = other_boids.iter().map(|other_boid| other_boid.pos());
        let too_close = other_positions.filter(|other_pos| 
        (boid.pos() - other_pos).absolute() <= self.parameters.avoidance_distance
        );
        let distances = too_close.map(|other_pos| boid.pos() - other_pos);
        let sum: Distance = distances.sum();
        let avoidance_time = Duration::from_secs(self.parameters.avoidance_time);
        return &sum / &avoidance_time;
    }

    fn match_velocity_velocity(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let velocities_sum: Velocity = other_boids.iter().map(|other_boid| other_boid.velocity()).sum();
        let num_other = other_boids.len() as f32;
        let velocity_mean = &velocities_sum / &num_other;
        return &(&velocity_mean - boid.velocity()) / &self.parameters.velocity_match_rate;
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
