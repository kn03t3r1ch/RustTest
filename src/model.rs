use self::ball::Ball;
use nannou::prelude::*;

pub mod ball;

const N_BALLS: usize = 16;
// #[derive(Debug)]
pub struct Model {
    pub last_pos: Vec2,
    pub balls: Vec<Ball>,
}

impl Model {
    pub fn new() -> Self {
        let mut balls = Vec::new();
        for i in 0..N_BALLS {
            let ball = Ball::new(
                pt2(
                    (rand::random::<f32>() - 0.5) * 1024.0,
                    (rand::random::<f32>() - 0.5) * 1024.0,
                ),
                (rand::random::<f32>() * 20.0) + 10.0,
                false,
                pt2(0.0, 0.0),
                hsl(rand::random::<f32>(), 1.0, 0.5),
            );
            balls.push(ball);
        }

        Self {
            last_pos: pt2(0.0, 0.0),
            balls,
        }
    }
}
