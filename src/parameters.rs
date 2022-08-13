use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct Parameters {
    pub fly_to_centre_time: u64,
    pub avoidance_distance: f32,
    pub avoidance_time: u64,
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