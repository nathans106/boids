use std::{time::Duration, collections::HashMap};

use pyo3::{pymethods, pyclass, Python, types::PyModule, PyResult, pymodule};

use crate::{velocity_calculator::VelocityCalculator, database::{Database, Id}, model::{Position, Boid}};

#[pyclass]
pub struct Simulation {
    database: Database,
    velocity_calculator: VelocityCalculator
}

#[pymethods]
impl Simulation {
    #[new]
    pub fn new(num_boids: i32, width: i32, height: i32) -> Self {
        Simulation{ database: Database::new(num_boids, width, height), velocity_calculator: VelocityCalculator::new() }
    }

    pub fn ids(&self) -> Vec<Id> {
        self.database.ids().into_iter().cloned().collect()
    }

    pub fn positions(&self) -> HashMap<Id, Position> {
        self.database.data().into_iter().map(|(id, boid)| (id.clone(), boid.pos().clone())).collect()
    }

    pub fn advance(&mut self, seconds: i32) {
        let one_second = Duration::from_secs(1);
        for _second in 0..seconds {
            let snapshot = self.database.data();
            let mut new_data = snapshot.clone();

            for (id, boid) in new_data.iter_mut() {
                let other_boids: Vec<&Boid> = snapshot.iter().filter(|(other_id, _other_boid)| *other_id != id).map(|(_other_id, other_boid)|other_boid).collect();
                let velocity = self.velocity_calculator.velocity(&boid, &other_boids);
                let distance = &velocity * &one_second;
                boid.move_(&distance);
            }

            self.database.set_data(new_data);
        }
    }
}

#[pymodule]
fn simulation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Simulation>()?;
    Ok(())
}