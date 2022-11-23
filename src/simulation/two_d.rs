use pyo3::pyclass;

use pyo3::prelude::*;

use crate::newtonian;

#[pyclass]
pub struct TwoDPosition {
    #[pyo3(get)]
    x: f32,
    #[pyo3(get)]
    y: f32,
}

impl TwoDPosition {
    pub fn from(newt: &newtonian::Position) -> Self {
        let data = newt.data();
        TwoDPosition {
            x: data[0],
            y: data[1],
        }
    }
}

#[pymodule]
fn two_d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TwoDPosition>()?;
    Ok(())
}
