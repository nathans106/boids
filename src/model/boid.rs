use crate::model::{Pos, Velocity};

pub struct Boid {
    pos: Pos
}

impl Boid {
    pub fn new() -> Self {
        Boid{pos: Pos{x: 0, y: 0}}
    }

    pub fn pos(&self) -> Pos {
        return Pos{x: 10, y: 10}
    }

    pub fn move_(&mut self, velocity: &Velocity) {
        self.pos += velocity
    }
}
