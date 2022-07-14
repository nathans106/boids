use crate::model::Boid;

pub struct Handler {

}

impl Handler {
    pub fn new() -> Handler {
        Handler{}
    }

    pub fn get_boids(&self) -> Vec<Box<dyn Boid>> {
        return vec![]
    }

    pub fn update(&self) {

    }
}
