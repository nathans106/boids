use std::{collections::HashMap, path::Path};

use pyo3::{pymethods, pyclass, Python, types::PyModule, PyResult, pymodule};

use crate::{database::{Database, Id}, newtonian::{Position}, parameters::parameters};

#[pyclass]
pub struct Simulation {
    database: Database
}

#[pymethods]
impl Simulation {
    #[new]
    pub fn new(num_boids: i32, width: i32, height: i32, parameters_file: &str) -> Self {
        Simulation{ database: Database::new(num_boids, width, height, &parameters(Path::new(parameters_file))) }
    }

    pub fn ids(&self) -> Vec<Id> {
        self.database.ids().into_iter().cloned().collect()
    }

    pub fn positions(&self) -> HashMap<Id, Position> {
        self.database.data().into_iter().map(|(id, boid)| (id.clone(), boid.pos().clone())).collect()
    }

    pub fn advance(&mut self, seconds: i32) {
        self.database.advance(seconds)
    }
}

#[pymodule]
fn simulation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Simulation>()?;
    Ok(())
}