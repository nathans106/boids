use std::collections::HashMap;

use rand::*;

use crate::model::Boid;
use crate::model::Position;
use crate::model::Velocity;

pub type Id = i32;
pub type Data = HashMap<Id, Boid>;


pub struct Database {
    boids: Data,
}

impl Database {
    pub fn new(num_boids: i32, width: i32, height: i32) -> Self {
        let mut rnd = rand::thread_rng();
        let boids = HashMap::from_iter((0..num_boids).map(|id| (id, Boid::new(Position::new(rnd.gen_range(0..width) as f32, rnd.gen_range(0..height) as f32), Velocity::new(rnd.gen_range(0..3) as f32, rnd.gen_range(0..3) as f32)))));
        Database{ boids: boids }
    }

    pub fn data(&self) -> &Data {
        &self.boids
    }

    pub fn set_data(&mut self, data: Data) {
        self.boids = data;
    }
    
    pub fn ids(&self) -> Vec<&Id> {
        self.boids.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_once() {
        let mut db = Database::new(1, 100, 100);
        db.advance(1);
        for pos in db.positions().values() {
            assert_eq!(pos.x, 1.0);
            assert_eq!(pos.y, 1.0);
        }
    }

    #[test]
    fn update_twice() {
        let mut db = Database::new(1, 100, 100);
        db.advance(1);
        db.advance(1);
        for pos in db.positions().values() {
            assert_eq!(pos.x, 2.0);
            assert_eq!(pos.y, 2.0);
        }
    }
}
