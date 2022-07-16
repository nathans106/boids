use std::collections::HashMap;

use crate::model::Boid;
use crate::model::Pos;
use crate::model::boids;

pub type Id = i32;

pub struct Handler {
    boids: HashMap<Id, Box<dyn Boid + Send>>
}

impl Handler {
    pub fn new(num_boids: i32) -> Handler {
        Handler{ boids: HashMap::from_iter((0..num_boids).map(|id| (id, Box::<dyn Boid + Send>::new(boids::Simple::new())))) }
    }

    pub fn ids(&self) -> Vec<Id> {
        self.boids.keys()
    }

    pub fn positions(&self) -> HashMap<Id, Pos> {
        HashMap::from_iter(&self.boids.iter().map(|(id, boid)| (id, boid.pos())))
    }

    pub fn update(&self) {

    }
}
