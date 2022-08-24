use self::{ass::Ass, ball::Ball};
use nannou::prelude::pt2;

pub mod ass;
pub mod ball;
#[derive(Debug)]
pub struct Model {
    pub ball: Ball,
    pub ass: Ass,
}

impl Model {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            ass: Ass { xy: pt2(0.0, 0.0) }, 
        }
    }
}
