//-------------------------------
// I wrote it like this with all the separat models to see how it works having a larger
// project structur
// it could have easlilier been done writing this completely in the main.rs file
// but this way it was a better learning experience for me :)
//-------------------------------
use crate::{BALL_COUNT, WINDOW_H, WINDOW_W};

use self::audio_file::Audio;
use self::ball::Ball;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
// I would like to check out another audio crate but haven't had the time left for it
// so I stuck with the very basic audio crate from nannou where no effects can be applied
pub mod audio_file;
pub mod ball;

// #[derive(Debug)]
pub struct Model {
    pub last_pos: Vec2,
    pub balls: Vec<Ball>,
    pub stream: audio::Stream<Audio>,
}

impl Model {
    pub fn new() -> Self {
        let mut balls = Vec::new();
        let mut counter = 0;
        let ball_count = BALL_COUNT;

        while counter < ball_count {
            let ball = Ball::new(
                pt2(
                    (rand::random::<f32>() - 0.5) * (WINDOW_W as f32 - 100.0),
                    (rand::random::<f32>() - 0.5) * (WINDOW_H as f32 - 100.0),
                ),
                (rand::random::<f32>() * 20.0) + 20.0,
                false,
                pt2(
                    (rand::random::<f32>() - 0.5) * 3.0,
                    (rand::random::<f32>() - 0.5) * 3.0,
                ),
                hsl(rand::random::<f32>(), 1.0, 0.5),
                0.5,
                false,
            );

            balls.push(ball);
            counter += 1;

            println!(
                "filling Vec at index: {:?}", // see the vector filling process
                balls.iter().enumerate().count()
            );

            let mut intersected = false;

            //----------
            // intersection testing when building array of balls at random position
            // and random radius so they don't intersect at startup
            if counter > 1 {
                for (i, other) in balls.iter_mut().enumerate() {
                    if i < counter - 1 {
                        let dist: f32 = ((other.position.x - ball.position.x).pow(2) as f32
                            + (other.position.y - ball.position.y).pow(2) as f32)
                            .sqrt();
                        if dist < other.size + ball.size {
                            intersected = true;
                            println!("intersection when building vec!"); // troubleshooting
                        }
                    }
                }
                if intersected == true {
                    balls.pop();
                    counter -= 1;
                }
            }
        }

        let audio_host = audio::Host::new();
        let sounds = vec![];
        let audio_model = Audio { sounds };
        let stream = audio_host
            .new_output_stream(audio_model)
            .render(audio)
            .build()
            .unwrap();
        stream.play().unwrap();
        //-----
        // testing if i can open a file in a directory this way!
        // let mut _file = File::open("./assets/sounds/dmk02__024-c0.wav").expect("Can't open file");
        Self {
            last_pos: pt2(0.0, 0.0),
            balls,
            stream,
        }
    }
}

fn audio(audio_file: &mut Audio, buffer: &mut Buffer) {
    let mut have_ended = vec![];
    let len_frames = buffer.len_frames();

    // Sum all of the sounds onto the buffer.
    for (i, sound) in audio_file.sounds.iter_mut().enumerate() {
        let mut frame_count = 0;
        let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);
        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                *sample += *file_sample;
            }
            frame_count += 1;
        }

        // If the sound yielded less samples than are in the buffer, it must have ended.
        if frame_count < len_frames {
            have_ended.push(i);
        }
    }

    // Remove all sounds that have ended.
    for i in have_ended.into_iter().rev() {
        audio_file.sounds.remove(i);
    }
}
