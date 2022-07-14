use crate::model::Pos;
use crate::py;

pub trait Boid {
    fn pos(&self) -> Pos;

    fn to_py_type(&self) -> py::Boid;
}
