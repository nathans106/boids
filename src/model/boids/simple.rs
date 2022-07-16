use crate::model::Boid;
use crate::model::Pos;

pub struct Simple {
}

impl Simple {
    pub fn new() -> Self {
        Simple{}
    }
}

impl Boid for Simple {
    fn pos(&self) -> Pos {
        return (10, 10)
    }
}
