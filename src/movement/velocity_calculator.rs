use std::{collections::HashMap, time::Duration};

use crate::{database::Id, model::{Boid, Velocity, Position}};

pub fn calculate_velocities(boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
    let mut velocities: HashMap<Id, Velocity> = HashMap::new();

    for (id, boid) in boids {
        let other_boids = boids.iter().filter(|(other_id, _other_boid)| other_id != &id);
        let other_positions: Vec<&Position> = other_boids.map(|(_other_id, other_boid)| other_boid.pos()).collect();
        
        let flock_centre = Position::centre(other_positions.into_iter());
        let velocity = &(&flock_centre - boid.pos()) / &Duration::from_secs(100);
        velocities.insert(id.clone(), velocity);
    }

    return velocities;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velocities() {
        let mut boids = HashMap::new();
        boids.insert(2, Boid::at(Position::new(0.0, 0.0)));

        let result = calculate_velocities(&boids);
        for velocity in result.values() {
            assert_eq!(velocity.x, 1.0);
            assert_eq!(velocity.x, 1.0);
        }
    }
}
