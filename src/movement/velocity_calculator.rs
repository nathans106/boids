use std::collections::HashMap;

use crate::{database::Id, model::{Boid, Velocity}};

pub fn calculate_velocities(boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
    let mut velocities = HashMap::new();

    for (id, _boid) in boids {
        velocities.insert(id.clone(), Velocity{ dx: 1, dy: 1 });
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
