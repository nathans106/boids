use crate::model::{Pos, Velocity};

pub struct Boid {
    pos: Pos
}

impl Boid {
    pub fn new() -> Self {
        Boid{pos: Pos{x: 0, y: 0}}
    }

    pub fn at(pos: Pos) -> Self {
        Boid{pos: pos}
    }

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

    pub fn move_(&mut self, velocity: &Velocity) {
        self.pos += velocity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_() {
        let mut boid = Boid::new();
        boid.move_(&Velocity { dx: 1, dy: 1 });
        let pos = boid.pos();
        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 1);
    }
}