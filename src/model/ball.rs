use nannou::prelude::{Hsl, Vec2};

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub position: Vec2,
    pub size: f32, 
    pub left_pressed: bool,
    pub velocity: Vec2,
    pub color: Hsl,
    pub mass: f32,
    pub glow: bool,
}

impl Ball {
    pub fn new(
        position: Vec2,
        size: f32,
        left_pressed: bool,
        velocity: Vec2,
        color: Hsl,
        mass: f32,
        glow: bool,
    ) -> Self {
        Ball {
            position,
            size,
            left_pressed,
            velocity,
            color,
            mass,
            glow,
        }
    }
}
