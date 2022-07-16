use std::collections::HashMap;

use crate::handler;
use crate::handler::Id;
use crate::model;

use pyo3::prelude::*;

#[pyclass]
struct Handler {
    handler: handler::Handler
}

#[pymethods]
impl Handler {
    #[new]
    fn new() -> Self {
        Handler{handler: handler::Handler::new(20)}
    }

    pub fn get_ids(&self) -> PyResult<Vec<Id>> {
        Ok(self.handler.ids())
    }

    pub fn get_positions(&self) -> PyResult<HashMap<Id, model::Pos>> {
        Ok(self.handler.positions())
    }
    
    pub fn update(&self) -> PyResult<()> {
        Ok(self.handler.update())
    }
}


#[pymodule]
fn boids(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Handler>()?;
    Ok(())
}
