use std::collections::HashMap;
use std::time::Duration;

use pyo3::prelude::*;
use rand::*;

use crate::model::Boid;
use crate::model::Position;
use crate::model::Velocity;
use crate::movement::velocity_calculator::VelocityCalculator;

pub type Id = i32;

#[pyclass]
pub struct Database {
    boids: HashMap<Id, Boid>,
    velocity_calculator: VelocityCalculator
}

#[pymethods]
impl Database {
    #[new]
    pub fn new(num_boids: i32, width: i32, height: i32) -> Self {
        let mut rnd = rand::thread_rng();
        let boids = HashMap::from_iter((0..num_boids).map(|id| (id, Boid::new(Position::new(rnd.gen_range(0..width) as f32, rnd.gen_range(0..height) as f32), Velocity::new()))));
        Database{ boids: boids, velocity_calculator: VelocityCalculator::new() }
    }

    pub fn ids(&self) -> Vec<Id> {
        self.boids.keys().cloned().collect()
    }

    pub fn positions(&self) -> HashMap<Id, Position> {
        HashMap::from_iter(self.boids.iter().map(|(id, boid)| (id.clone(), boid.pos().clone())))
    }

    pub fn advance(&mut self, seconds: i32) {
        let velocities = self.velocity_calculator.velocities(&self.boids);
        for _second in 0..seconds {
            for (id, boid) in self.boids.iter_mut() {
                let velocity = &velocities[id];
                let distance = velocity * &Duration::from_secs(1);
                boid.move_(&distance);
            }
        }

    }
}

#[pymodule]
fn database(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Database>()?;
    Ok(())
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
