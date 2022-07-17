use pyo3::prelude::*;

#[pyclass]
pub struct Pos {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32
}
