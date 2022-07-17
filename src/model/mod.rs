mod boid;
pub use boid::Boid;
mod pos;
pub use pos::Pos;
pub use pos::Velocity;

use pyo3::prelude::*;

#[pymodule]
fn model(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pos>()?;
    Ok(())
}
