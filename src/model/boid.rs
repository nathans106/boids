use crate::model::{Position, Distance};

use super::Velocity;

#[derive(Clone)]
pub struct Boid {
    pos: Position,
    velocity: Velocity
}

impl Boid {
    pub fn new(pos: Position, velocity: Velocity) -> Self {
        Boid{pos: pos, velocity: velocity}
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn velocity(&self) -> &Velocity {
        &self.velocity
    }

    pub fn move_(&mut self, distance: &Distance) {
        self.pos += distance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_() {
        let mut boid = Boid::new(Position::origin(), Velocity::new());
        boid.move_(&Velocity { x: 1.0, x: 1.0 });
        let pos = boid.pos();
        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 1);
    }
}