use nannou::prelude::{pt2, Hsl, Vec2};
// use nannou::prelude::*;
pub struct Ball {
    pub position: Vec2,
    pub size: f32,
    pub left_pressed: bool,
    pub velocity: Vec2,
    pub color: Hsl,
}
// const N_THINGS: usize = 16;
impl Ball {
    pub fn new(position: Vec2, size: f32, left_pressed: bool, velocity: Vec2, color: Hsl) -> Self {
        Ball {
            position,
            size,
            left_pressed,
            velocity,
            color,
        }
    }
}
