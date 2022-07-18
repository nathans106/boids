use std::collections::HashMap;

use pyo3::prelude::*;

use crate::model::Boid;
use crate::model::Pos;
use crate::movement::velocity_calculator::calculate_velocities;

pub type Id = i32;

#[pyclass]
pub struct Database {
    boids: HashMap<Id, Boid>
}

#[pymethods]
impl Database {
    #[new]
    pub fn new(num_boids: i32) -> Self {
        Database{ boids: HashMap::from_iter((0..num_boids).map(|id| (id, Boid::new()))) }
    }

    pub fn ids(&self) -> Vec<Id> {
        self.boids.keys().cloned().collect()
    }

    pub fn positions(&self) -> HashMap<Id, Pos> {
        HashMap::from_iter(self.boids.iter().map(|(id, boid)| (id.clone(), boid.pos().clone())))
    }

    pub fn update(&mut self) {
        let velocities = calculate_velocities(&self.boids);
        for (id, boid) in self.boids.iter_mut() {
            boid.move_(&velocities[id]);
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
    fn update_once() {
        let mut db = Database::new(1);
        db.update();
        for pos in db.positions().values() {
            assert_eq!(pos.x, 1);
            assert_eq!(pos.y, 1);
        }
    }

    #[test]
    fn update_twice() {
        let mut db = Database::new(1);
        db.update();
        db.update();
        for pos in db.positions().values() {
            assert_eq!(pos.x, 2);
            assert_eq!(pos.y, 2);
        }
    }
}
