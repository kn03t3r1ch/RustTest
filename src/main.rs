// This is no longer needed because we imported Ball already within the Model crate
// use crate::model::ball::Ball;
use crate::model::Model;
use model::audio_file::Audio;
use model::ball::Ball;
use nannou_audio as audio;
use nannou_audio::Buffer;
// use model::ball::{self, Ball};
use nannou::prelude::*;
mod model;
fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .size(1920, 1080)
        .event(event_a)
        .build()
        .unwrap();

    Model::new()
}

fn event_a(app: &App, model: &mut Model, event: WindowEvent) {
    // println!("event_a {:?}", model.balls.velocity);
}

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
    // -----------------
    // testing with vec of BAlls
    for (count, other) in model.balls.iter_mut().enumerate() {
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

        //---------
        // Testing implementing intersection here

        // Bounce of screen sides
        // if (other.position.x > rect.right() - other.size)
        //     || (other.position.x < rect.left() + other.size)
        // {
        //     other.velocity.x *= -1.0;
        // }
        // if (other.position.y > rect.top() - other.size)
        //     || (other.position.y < rect.bottom() + other.size)
        // {
        //     other.velocity.y *= -1.0;
        // }
        // mby with slider adjust velocity increase also to make the whole thing look
        // And mby also I could build this into an slider that one can use on top of the renderer
        // other.velocity *= 1.0;
        // model.last_pos = mouse_pos;
    }

    for i in 0..len {
        // intersection testing
        for j in i + 1..len {
            if model.balls[i].position.distance(model.balls[j].position)
                < (model.balls[i].size + model.balls[j].size)
            {
                // println!("intersection at: {:?}", model.balls[j].position);
                // model.balls[i].color = hsl(random_f32(), 1.0, 0.5);
                // model.balls[j].color = hsl(random_f32(), 1.0, 0.5);
                // resolve_collision(model.balls[i], model.balls[j]);
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

        // mby Slider for velocity for eva?!?!
        model.balls[i].velocity *= 0.9999;
        model.last_pos = mouse_pos;
    }
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

// fn audio(audio: &mut Audio, buffer: &mut Buffer){

// }

// this is for testing purposes how to implement sound
// But this is only for a key press -> I need to refactor this
// for making it usable if objects collide
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        // Start playing another instance of the sound.
        // Key::Space => {
        //     let assets = app.assets_path().expect("could not find assets directory");
        //     let path = assets.join("sounds").join("thumbpiano.wav");
        //     let sound = audrey::open(path).expect("failed to load sound");
        //     model
        //         .stream
        //         .send(move |audio| {
        //             audio.sounds.push(sound);
        //         })
        //         .ok();
        //     println!("space pressed!");
        // }
        // _ => {}
        Key::Space => {
            let assets = app.assets_path().expect("could not find assets directory");
            let random_value = random_range(0, 3);
            let path = assets
                .join("sounds_2")
                .join(format!("dmk02__{}.wav", random_value));
            let sound = audrey::open(path).expect("failed to fuckin load the sound");
            model
                .stream
                .send(move |audio| {
                    audio.sounds.push(sound);
                })
                .ok();
        }
        _ => {}
    }
}
