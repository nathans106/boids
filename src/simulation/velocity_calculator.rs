use crate::{model::{Boid, Velocity}, velocity_calculators::{Calculator, AvoidCollision, FlockToCentre, MatchVelocity}, parameters::Parameters};

#[derive(Clone)]
pub struct VelocityCalculator {
    max_velocity: f32,
    vision_distance: f32,

    avoid_collision: AvoidCollision,
    flock_to_centre: FlockToCentre,
    match_velocity: MatchVelocity
}

impl VelocityCalculator {
    pub fn new(parameters: &Parameters) -> Self {
        VelocityCalculator {
            max_velocity: parameters.boid.max_velocity,
            vision_distance: parameters.boid.vision_distance,
            avoid_collision : parameters.avoid_collision.clone(),
            flock_to_centre: parameters.flock_to_centre.clone(),
            match_velocity: parameters.match_velocity.clone()
        }
    }

    pub fn velocity(&self, boid: &Boid, other_boids: &[&Boid]) -> Velocity {
        let mut velocity = Velocity::new(0.0, 0.0);

        let visible_boids: Vec<&Boid> = other_boids.into_iter().filter(|other_boid| (other_boid.pos() - boid.pos()).abs() <= self.vision_distance).cloned().collect();

        velocity += &self.avoid_collision.calculate(&boid, &visible_boids);
        velocity += &self.flock_to_centre.calculate(&boid, &visible_boids);
        velocity += &self.match_velocity.calculate(&boid, &visible_boids);

        let abs_velocity = velocity.abs();

        if abs_velocity > self.max_velocity {
            return &(&velocity / &abs_velocity) * &self.max_velocity;
        }

        return velocity;
    }
}
