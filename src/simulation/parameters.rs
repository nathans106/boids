use pyo3::{pyclass, pymethods};

#[pyclass]
#[derive(Clone)]
pub struct Parameters {
    #[pyo3(get, set)]
    pub fly_to_centre_time: u64,
    #[pyo3(get, set)]
    pub avoidance_distance: f32,
    #[pyo3(get, set)]
    pub avoidance_time: u64,
    #[pyo3(get, set)]
    pub velocity_match_rate: f32
}

#[pymethods]
impl Parameters {
    #[new]
    pub fn new() -> Self {
        Parameters {
            fly_to_centre_time: 100,
            avoidance_distance: 40.0,
            avoidance_time: 50,
            velocity_match_rate: 3.0
        }
    }
}
