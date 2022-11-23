use std::{collections::HashMap, path::Path};

use pyo3::{pyclass, pymethods, pymodule, types::PyModule, PyResult, Python};

use crate::{
    database::{Database, Id},
    parameters::parameters,
    two_d,
};

pub const DIMENSIONS: usize = 2;
pub const TICK: f32 = 1.0;

#[pyclass]
pub struct Simulation {
    database: Database,
}

#[pymethods]
impl Simulation {
    #[new]
    pub fn new(num_boids: i32, width: i32, height: i32, parameters_file: &str) -> Self {
        Simulation {
            database: Database::new(
                num_boids,
                width,
                height,
                &parameters(Path::new(parameters_file)),
            ),
        }
    }

    pub fn ids(&self) -> Vec<Id> {
        self.database.ids().into_iter().cloned().collect()
    }

    pub fn positions(&self) -> HashMap<Id, two_d::TwoDPosition> {
        self.database
            .data()
            .into_iter()
            .map(|(id, boid)| (id.clone(), two_d::TwoDPosition::from(boid.pos())))
            .collect()
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
