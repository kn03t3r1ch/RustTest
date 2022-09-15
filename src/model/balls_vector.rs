// use nannou::{
//     color::encoding::Srgb,
//     prelude::{pt2, Vec2, rgb},
// };
use nannou::prelude::*;
pub struct Thing {
    pub position: Vec2,
    pub size: f32,
    pub left_pressed: bool,
    pub velocity: Vec2,
    pub color: Hsl,
}
// const N_THINGS: usize = 16;
impl Thing {
    pub fn new(position: Vec2, size: f32, left_pressed: bool, velocity: Vec2, color: Hsl) -> Self {
        Thing {
            position,
            size,
            left_pressed,
            velocity,
            color,
        }
    }
}
