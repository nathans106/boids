use std::collections::HashMap;

use crate::{database::Id, model::{Boid, Velocity, Pos}};

pub fn calculate_velocities(boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
    let mut velocities = HashMap::new();

    for (id, boid) in boids {
        let other_boids = boids.iter().filter(|(other_id, _other_boid)| other_id != &id);
        let other_positions: Vec<&Pos> = other_boids.map(|(_other_id, other_boid)| other_boid.pos()).collect();
        
        let flock_centre = Pos::centre(other_positions.into_iter());
        let velocity = (flock_centre - boid.pos()) / &100;
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
        boids.insert(2, Boid::new());

        let result = calculate_velocities(&boids);
        for velocity in result.values() {
            assert_eq!(velocity.dx, 1);
            assert_eq!(velocity.dy, 1);
        }
    }
}
