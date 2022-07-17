use std::collections::HashMap;

use crate::{database::Id, model::{Boid, Velocity}};

pub fn calculate_velocities(boids: &HashMap<Id, Boid>) -> HashMap<Id, Velocity> {
    let mut velocities = HashMap::new();

    for (id, _boid) in boids {
        velocities.insert(id.clone(), Velocity{ dx: 1, dy: 1 });
    }

    return velocities;
}
