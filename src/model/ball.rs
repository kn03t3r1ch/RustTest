use nannou::prelude::{Hsl, Vec2};
// use nannou::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub position: Vec2,
    pub size: f32, // in this case size equals mass for intercollision detections between balls
    pub left_pressed: bool,
    pub velocity: Vec2,
    pub color: Hsl,
    pub mass: f32,
}
// const N_THINGS: usize = 16;
impl Ball {
    pub fn new(
        position: Vec2,
        size: f32,
        left_pressed: bool,
        velocity: Vec2,
        color: Hsl,
        mass: f32,
    ) -> Self {
        Ball {
            position,
            size,
            left_pressed,
            velocity,
            color,
            mass,
        }
    }
}
