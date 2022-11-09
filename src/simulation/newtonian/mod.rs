mod acceleration;
pub use acceleration::Acceleration;
mod force;
pub use force::Force;
mod mass;
pub use mass::Mass;
mod position;
pub use position::Position;
mod distance;
pub use distance::Distance;
mod vector;
pub use vector::Vector;
mod velocity;
pub use velocity::Velocity;

use pyo3::prelude::*;

#[pymodule]
fn model(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Position>()?;
    Ok(())
}
