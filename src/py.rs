use crate::handler;
use crate::model;

use pyo3::prelude::*;

#[pyclass]
pub enum BoidType {
    Dummy1,
    Dummy2
}

#[pyclass]
pub struct Boid {
    pub pos: model::Pos,
    pub type_: BoidType
}

#[pyclass]
struct Handler {
    handler: handler::Handler
}

#[pymethods]
impl Handler {
    #[new]
    fn new() -> Self {
        Handler{handler: handler::Handler::new()}
    }

    pub fn get_boids(&self) -> PyResult<Vec<Boid>> {
        Ok(self.handler.get_boids().iter().map(|boid| boid.to_py_type()).collect())
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
