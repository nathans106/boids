use std::{
    iter::Sum,
    ops::{AddAssign, Div, Mul, Sub},
};

use serde::Deserialize;

use crate::simulation::DIMENSIONS;

use super::Scalar;

#[derive(Clone, Copy, Deserialize)]
pub struct Vector {
    values: [f32; DIMENSIONS],
}

impl Vector {
    pub fn new(values: [f32; DIMENSIONS]) -> Self {
        Vector { values }
    }

    pub fn data(&self) -> &[f32; DIMENSIONS] {
        &self.values
    }

    pub fn origin() -> Self {
        Self {
            values: [0.0; DIMENSIONS],
        }
    }

    pub fn abs(&self) -> f32 {
        self.values
            .iter()
            .map(|val| val.powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn mean(vectors: &[&Self]) -> Self
    where
        Self: Sized,
    {
        let num_vectors = vectors.len() as Scalar;
        let sum = vectors.iter().map(|v| **v).sum::<Self>();
        &sum / &num_vectors
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        for (idx, value) in self.values.iter_mut().enumerate() {
            *value += rhs.values[idx];
        }
    }
}

impl Div<&Scalar> for &Vector {
    type Output = Vector;

    fn div(self, rhs: &Scalar) -> Self::Output {
        let values = self.values.map(|num| num / rhs);
        Vector::new(values)
    }
}

impl Div<&Vector> for &Vector {
    type Output = Vector;

    fn div(self, rhs: &Vector) -> Self::Output {
        let values = self.values.zip(rhs.values).map(|(num, den)| num / den);
        Vector::new(values)
    }
}

impl Mul<&Scalar> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        let values = self.values.map(|v| v * rhs);
        Vector::new(values)
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        let values = self.values.zip(rhs.values).map(|(l, r)| l - r);
        Vector::new(values)
    }
}

impl Sum<Vector> for Vector {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Vector {
        let mut result = Vector::origin();
        for v in iter {
            result += &v;
        }

        result
    }
}
