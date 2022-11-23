use crate::{
    boid::Boid,
    force_calculators::{AvoidCollision, Calculator, FlockToCentre, MatchVelocity},
    newtonian::Force,
    parameters::Parameters,
};

#[derive(Clone)]
pub struct ForceCalculator {
    max_velocity: f32,
    vision_distance: f32,

    avoid_collision: AvoidCollision,
    flock_to_centre: FlockToCentre,
    match_velocity: MatchVelocity,
}

impl ForceCalculator {
    pub fn new(parameters: &Parameters) -> Self {
        ForceCalculator {
            max_velocity: parameters.boid.max_velocity,
            vision_distance: parameters.boid.vision_distance,
            avoid_collision: parameters.avoid_collision.clone(),
            flock_to_centre: parameters.flock_to_centre.clone(),
            match_velocity: parameters.match_velocity.clone(),
        }
    }

    pub fn force(&self, boid: &Boid, other_boids: &[&Boid]) -> Force {
        let mut force = Force::origin();

        let visible_boids: Vec<&Boid> = other_boids
            .into_iter()
            .filter(|other_boid| (other_boid.pos() - boid.pos()).abs() <= self.vision_distance)
            .cloned()
            .collect();

        force += &self.avoid_collision.calculate(&boid, &visible_boids);
        force += &self.flock_to_centre.calculate(&boid, &visible_boids);
        force += &self.match_velocity.calculate(&boid, &visible_boids);

        let abs_velocity = force.abs();

        if abs_velocity > self.max_velocity {
            return &(&force / &abs_velocity) * &self.max_velocity;
        }

        return force;
    }
}
