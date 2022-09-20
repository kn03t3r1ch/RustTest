use self::ball::Ball;
use nannou::prelude::*;

pub mod ball;

// const N_BALLS: usize = 30;
// #[derive(Debug)]
pub struct Model {
    pub last_pos: Vec2,
    pub balls: Vec<Ball>,
}

impl Model {
    pub fn new() -> Self {
        let mut balls = Vec::new();
        let mut counter = 0;
        let ball_count = 5;

        while counter < ball_count {
            let ball = Ball::new(
                pt2(
                    (rand::random::<f32>() - 0.5) * 512.0,
                    (rand::random::<f32>() - 0.5) * 512.0,
                ),
                (rand::random::<f32>() * 50.0) + 20.0,
                false,
                pt2(0.0, 0.0),
                // hsl(0.3, 1.0, 0.5),
                hsl(rand::random::<f32>(), 1.0, 0.5),
            );

            balls.push(ball);
            counter += 1;

            println!(
                "filling Vec at index: {:?}",
                balls.iter().enumerate().count()
            );

            let mut intersected = false;

            if counter > 1 {
                for (i, other) in balls.iter_mut().enumerate() {
                    if i < counter - 1 {
                        let dist: f32 = ((other.position.x - ball.position.x).pow(2) as f32
                            + (other.position.y - ball.position.y).pow(2) as f32)
                            .sqrt();
                        if dist < other.size + ball.size {
                            intersected = true;
                            println!("intersection!")
                        }
                    }
                }
                if intersected == true {
                    balls.pop();
                    counter -= 1;
                }
            }
        }

        Self {
            last_pos: pt2(0.0, 0.0),
            balls,
        }
    }
}
