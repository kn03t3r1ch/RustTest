use nannou::prelude::{Vec2, pt2};

#[derive(Debug)]
pub struct Ball {
    pub xy: Vec2,
    pub ball_size: f32,
    pub left_pressed: bool,
}

impl Ball {
    pub fn new() -> Self {
        let xy = pt2(0.0, 0.0);
        let ball_size = 25.0;
        let left_pressed = false;

        Self {
            xy,
            ball_size,
            left_pressed,
        }
    }
}
