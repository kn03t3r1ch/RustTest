use nannou::prelude::{pt2, Vec2};

pub struct Thing {
    pub position: Vec2,
}
// const N_THINGS: usize = 16;
impl Thing {
    pub fn new(p: Vec2) -> Self {
        Thing { position: p }
    }
}
