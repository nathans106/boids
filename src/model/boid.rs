use crate::model::{Position, Distance};

pub struct Boid {
    pos: Position
}

impl Boid {
    pub fn at(pos: Position) -> Self {
        Boid{pos: pos}
    }

    pub fn pos(&self) -> &Position {
        &self.pos
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
        let mut boid = Boid::new();
        boid.move_(&Velocity { dx: 1, dy: 1 });
        let pos = boid.pos();
        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 1);
    }
}