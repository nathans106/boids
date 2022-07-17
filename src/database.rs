use std::collections::HashMap;

use pyo3::prelude::*;

use crate::model::Boid;
use crate::model::Pos;

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
        HashMap::from_iter(self.boids.iter().map(|(id, boid)| (id.clone(), boid.pos())))
    }

    pub fn update(&self) {

    }
}

#[pymodule]
fn database(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Database>()?;
    Ok(())
}