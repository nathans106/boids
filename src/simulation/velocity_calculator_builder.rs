
use serde::Deserialize;

use toml;

use std::path::Path;

use crate::{velocity_calculators::{AvoidCollision, FlockToCentre, MatchVelocity}, velocity_calculator::{VelocityCalculator}};

#[derive(Deserialize)]
struct Parameters {
    avoid_collision: AvoidCollision,
    flock_to_centre: FlockToCentre,
    match_velocity: MatchVelocity
}

pub fn build_velocity_calculator(parameters_file: &Path) -> VelocityCalculator {
    let file_str = std::fs::read_to_string(&parameters_file).unwrap();
    let parameters: Parameters = toml::from_str(&file_str).unwrap();

    let mut velocity_calculator = VelocityCalculator::new();
    velocity_calculator.add_calculator(Box::new(parameters.avoid_collision));
    velocity_calculator.add_calculator(Box::new(parameters.flock_to_centre));
    velocity_calculator.add_calculator(Box::new(parameters.match_velocity));

    return velocity_calculator;
}
