use crate::model::Model;
use model::ball::Ball;
use nannou::prelude::*;
use std::time::{self, Duration, SystemTime};
mod model;

const BALL_COUNT: usize = 25;
const SOUND_COUNT: usize = 72;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1920, 1080)
        .event(event_a)
        .build()
        .unwrap();

    Model::new()
}

fn event_a(_app: &App, _model: &mut Model, _event: WindowEvent) {}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse_pressed = app.mouse.buttons.left().is_down();
    // let mouse_pressed_right = app.mouse.buttons.right().is_down();
    let rect = app.window_rect();
    let mouse_pos = app.mouse.position().clamp(
        pt2(rect.left(), rect.bottom()),
        pt2(rect.right(), rect.top()),
    );
    let mouse_delta_pos = mouse_pos - model.last_pos;
    let len = model.balls.len();

    //-------------
    // intersection testing
    for i in 0..len {
        for j in i + 1..len {
            if model.balls[i].position.distance(model.balls[j].position)
                < (model.balls[i].size + model.balls[j].size)
            {
                play_rand_sound(app, model, model.balls[i].position);
                (model.balls[i], model.balls[j]) =
                    resolve_collision(model.balls[i], model.balls[j]);
            }
        }
        //Bouncing of the sides
        if (model.balls[i].position.x > rect.right() - model.balls[i].size)
            || (model.balls[i].position.x < rect.left() + model.balls[i].size)
        {
            model.balls[i].velocity.x *= -1.0;
        }
        if (model.balls[i].position.y > rect.top() - model.balls[i].size)
            || (model.balls[i].position.y < rect.bottom() + model.balls[i].size)
        {
            model.balls[i].velocity.y *= -1.0;
        }

        // mby Slider for adjusting velocity while having the appication open?!?!?
        model.balls[i].velocity *= 0.999;
        model.last_pos = mouse_pos;
    }

    // -----------------
    // testing with vec of BAlls
    for (_count, other) in model.balls.iter_mut().enumerate() {
        if mouse_pressed == true && other.position.distance(mouse_pos) < (other.size) {
            // model.xy = mouse_pos;
            other.left_pressed = true;
        }
        if other.left_pressed == true {
            other.position = mouse_pos;
        }
        if mouse_pressed == false && other.left_pressed == true {
            other.left_pressed = false;
            other.velocity = mouse_delta_pos;
        }
        other.position += other.velocity;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(0.0, 0.0, 0.0, 0.03);

    for ball in model.balls.iter() {
        draw.ellipse()
            .xy(ball.position)
            .radius(ball.size)
            .color(ball.color);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn rotate(velocity: Vec2, angle: f32) -> Vec2 {
    let rotated_velocities: Vec2 = pt2(
        velocity.x * angle.cos() - velocity.y * angle.sin(),
        velocity.x * angle.sin() + velocity.y * angle.cos(),
    );
    rotated_velocities
}

fn resolve_collision(mut particle: Ball, mut other_particle: Ball) -> (Ball, Ball) {
    let x_velocity_diff = particle.velocity.x - other_particle.velocity.x;
    let y_velocity_diff = particle.velocity.y - other_particle.velocity.y;

    let x_dist = other_particle.position.x - particle.position.x;
    let y_dist = other_particle.position.y - particle.position.y;

    // Prevent accidental overlap of particles
    if x_velocity_diff * x_dist + y_velocity_diff * y_dist >= 0.0 {
        // Grab angle between the two colliding particles
        let angle = -(other_particle.position.y - particle.position.y)
            .atan2(other_particle.position.x - particle.position.x);

        // Store mass in var for better readability in collision equation
        let m1 = particle.mass;
        let m2 = other_particle.mass;

        // Velocity before equation
        let u1 = rotate(particle.velocity, angle);
        let u2 = rotate(other_particle.velocity, angle);

        // Velocity after 1d collision equation
        let v1 = pt2(
            u1.x * (m1 - m2) / (m1 + m2) + u2.x * 2.0 * m2 / (m1 + m2),
            u1.y,
        );
        let v2 = pt2(
            u2.x * (m1 - m2) / (m1 + m2) + u1.x * 2.0 * m2 / (m1 + m2),
            u2.y,
        );

        // Final velocity after rotating axis back to original location
        let v_final1 = rotate(v1, -angle);
        let v_final2 = rotate(v2, -angle);

        // Swap particle velocities for realistic bounce effect
        particle.velocity.x = v_final1.x;
        particle.velocity.y = v_final1.y;

        other_particle.velocity.x = v_final2.x;
        other_particle.velocity.y = v_final2.y;
    }
    (particle, other_particle)
}

fn play_rand_sound(app: &App, model: &mut Model, _position: Vec2) {
    let assets = app.assets_path().expect("could not find assets directory");
    let random_value = random_range(0, SOUND_COUNT);
    let _pan = _position.x; // I would liked to have a little bit more audio processing
    let _vol = _position.y; // here but then I would have to rewrite the whole playback
                            // function and audiofunction  because the audio crate of nannou
                            // provides only basic functionality -> not even paning and
                            // adjusting volume is possible
    let path = assets
        .join("sounds")
        .join(format!("dmk02__{}.wav", random_value));
    let sound = audrey::open(path).expect("failed to load the sound");
    model
        .stream
        .send(move |audio| {
            audio.sounds.push(sound);
        })
        .ok();
}

//-----------
// this is noch working as intended...
// fn decay_function(mut trigger: bool) -> f32 {
//     let mut lightness_add = 0.0;
//     let length = 250;
//     let mut x = length;
//     let rate = 0.5 * (1.0 - 0.005);
//     if trigger {
//         x = 0;
//         // println!("lightness_add: {}", lightness_add);
//         trigger = false;
//     }

//     while x < length {
//         lightness_add = 0.5 * (1.0 - 0.005).powi(x);
//         x += 1;
//         // println!("x: {}", lightness_add);
//     }

//     trigger = false;

//     lightness_add as f32

//     // println!("lightness_add: {}", lightness_add);
// }
