use std::ops::Add;

use nannou::prelude::{pt2, Vec2};

#[derive(Debug)]
pub struct Ball {
    pub xy: Vec2,
    pub ball_size: f32,
    pub left_pressed: bool,
    pub velocity: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        let xy = pt2(0.0, 0.0);
        // let ball_size = 25.0;
        let left_pressed = false;

        Self {
            xy,
            ball_size: 25.0,
            left_pressed,
            velocity: pt2(0.0, 0.0),
        }
    }
}

// impl Add for Ball {
//     type Output = Ball;

//     fn add(self, rhs: Self) -> Self::Output {

//     }
// }

// for implementing vec 2 of mby 2 mouse.positions this is the implementation of it
// fn add(self, rhs: Self) -> Self::Output {
//     Vec2{
//         x: self.x + rhs.x,
//         y: self.y + rhs.y,
//     }
// }
