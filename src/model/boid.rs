use crate::model::Pos;

pub struct Boid {
}

impl Boid {
    pub fn new() -> Self {
        Boid{}
    }

    pub fn pos(&self) -> Pos {
        return (10, 10)
    }
}
