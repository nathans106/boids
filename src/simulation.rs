use crate::boid::Boid;

#[derive(Default)]
pub struct Simulation {
    boids: Vec<Boid>,
}

impl Simulation {
    pub fn boids(&self) -> &Vec<Boid> {
        &self.boids
    }
}
