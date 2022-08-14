mod boid;
pub use boid::Boid;
mod position;
pub use position::Position;
mod distance;
pub use distance::Distance;
mod velocity;
pub use velocity::Velocity;

use pyo3::prelude::*;

#[pymodule]
fn model(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Position>()?;
    Ok(())
}
