use crate::boid::Boid;
use crate::config::CONFIG;

pub struct Simulation {
    boids: Vec<Boid>,
}

impl Default for Simulation {
    fn default() -> Self {
        let num_boids = CONFIG.num_boids;
        let boids = (0..num_boids).map(|_i| Boid::default()).collect();

        Self { boids }
    }
}

impl Simulation {
    pub fn boids(&self) -> &Vec<Boid> {
        &self.boids
    }
}
