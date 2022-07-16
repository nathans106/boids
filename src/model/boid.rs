use crate::model::Pos;

pub trait Boid {
    fn pos(&self) -> Pos;
}
