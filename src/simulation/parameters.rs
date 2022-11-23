use serde::Deserialize;

use toml;

use std::path::Path;

use crate::{
    force_calculators::{AvoidCollision, FlockToCentre, MatchVelocity},
    newtonian::Mass,
};

#[derive(Deserialize)]
pub struct VelocityParameters {
    pub mass: Mass,
    pub max_velocity: f32,
    pub vision_distance: f32,
}

#[derive(Deserialize)]
pub struct Parameters {
    pub boid: VelocityParameters,
    pub avoid_collision: AvoidCollision,
    pub flock_to_centre: FlockToCentre,
    pub match_velocity: MatchVelocity,
}

pub fn parameters(parameters_file: &Path) -> Parameters {
    let file_str = std::fs::read_to_string(&parameters_file).unwrap();
    return toml::from_str(&file_str).unwrap();
}
