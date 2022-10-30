use crate::{model::{Boid, Velocity}, velocity_calculators::Calculator};

type Calculators = Vec<Box<dyn Calculator + Send>>;

pub struct VelocityCalculator {
    calculators: Calculators,
    max_velocity: f32,
    vision_distance: f32
}

impl VelocityCalculator {
    pub fn new(max_velocity: f32, vision_distance: f32) -> Self {
        VelocityCalculator {
            calculators: vec![],
            max_velocity: max_velocity,
            vision_distance: vision_distance
        }
    }

    pub fn add_calculator(&mut self, calculator: Box<dyn Calculator + Send>) {
        self.calculators.push(calculator);
    }

    pub fn velocity(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let mut velocity = Velocity::new(0.0, 0.0);

        let visible_boids: Vec<&Boid> = other_boids.into_iter().filter(|other_boid| (other_boid.pos() - boid.pos()).abs() <= self.vision_distance).cloned().collect();

        for calculator in &self.calculators {
            velocity += &calculator.as_ref().calculate(&boid, &visible_boids);
        }

        let abs_velocity = velocity.abs();

        if abs_velocity > self.max_velocity {
            return &(&velocity / &abs_velocity) * &self.max_velocity;
        }

        return velocity;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velocities() {
        let mut boids = HashMap::new();
        boids.insert(2, Boid::at(Position::new(0.0, 0.0)));

        let velocity_calculator = VelocityCalculator::new();
        let result = velocity_calculator.velocities(&boids);
        for velocity in result.values() {
            assert_eq!(velocity.x, 1.0);
            assert_eq!(velocity.x, 1.0);
        }
    }
}
