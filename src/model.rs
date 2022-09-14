use self::{ass::Ass, ball::Ball, balls_vector::Thing};
use crate::app::App;
// use nannou::prelude::{pt2, Vec2};
use nannou::prelude::*;

pub mod ass;
pub mod ball;
pub mod balls_vector;

const N_BALLS: usize = 16;
// #[derive(Debug)]
pub struct Model {
    pub ball: Ball,
    pub ass: Ass,
    pub last_pos: Vec2,
    pub things: Vec<Thing>,
}

impl Model {
    pub fn new() -> Self {
        let mut things = Vec::new();
        for i in 0..N_BALLS {
            let thing = Thing::new(pt2(
                (rand::random::<f32>() - 0.5) * 1024.0,
                (rand::random::<f32>() - 0.5) * 1024.0,
            ));
            things.push(thing);
        }

        Self {
            ball: Ball::new(),
            ass: Ass { xy: pt2(0.0, 0.0) },
            last_pos: pt2(0.0, 0.0),
            things,
        }
    }
}
