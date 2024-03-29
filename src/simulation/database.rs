use std::collections::HashMap;

use rand::*;

use crate::boid::Boid;
use crate::newtonian::Position;
use crate::newtonian::Velocity;
use crate::parameters::Parameters;
use crate::simulation::TICK;

pub type Id = i32;
pub type Data = HashMap<Id, Boid>;

pub struct Database {
    boids: Data,
}

impl Database {
    pub fn new(num_boids: i32, width: i32, height: i32, parameters: &Parameters) -> Self {
        let mut rnd = rand::thread_rng();
        let boids = HashMap::from_iter((0..num_boids).map(|id| {
            (
                id,
                Boid::new(
                    Position::new([
                        rnd.gen_range(0..width) as f32,
                        rnd.gen_range(0..height) as f32,
                    ]),
                    Velocity::new([rnd.gen_range(-2..3) as f32, rnd.gen_range(-2..3) as f32]),
                    parameters,
                ),
            )
        }));
        Database { boids }
    }

    pub fn data(&self) -> &Data {
        &self.boids
    }

    pub fn ids(&self) -> Vec<&Id> {
        self.boids.keys().collect()
    }

    pub fn advance(&mut self, seconds: i32) {
        for _second in 0..seconds {
            let snapshot = self.boids.clone();

            for (id, boid) in self.boids.iter_mut() {
                let other_boids: Vec<&Boid> = snapshot
                    .iter()
                    .filter(|(other_id, _other_boid)| *other_id != id)
                    .map(|(_other_id, other_boid)| other_boid)
                    .collect();
                boid.update(&other_boids);
                boid.advance(&TICK);
            }
        }
    }
}
