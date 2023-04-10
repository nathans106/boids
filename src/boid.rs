pub type Coord = (f32, f32);

#[derive(Default)]
pub struct Boid {
    coord: Coord,
}

impl Boid {
    pub fn coord(&self) -> &Coord {
        &self.coord
    }
}
