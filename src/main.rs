// This is no longer needed because we imported Ball already within the Model crate
// use crate::model::ball::Ball;
use crate::model::Model;
// use model::ball::{self, Ball};
use nannou::prelude::*;
mod model;
fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().event(event_a).build().unwrap();
    Model::new()
}

fn event_a(app: &App, model: &mut Model, event: WindowEvent) {
    // println!("event_a {:?}", model.ball.velocity);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse_pressed = app.mouse.buttons.left().is_down();
    let mouse_pressed_right = app.mouse.buttons.right().is_down();
    let rect = app.window_rect();
    let mouse_pos = app.mouse.position().clamp(
        pt2(rect.left(), rect.bottom()),
        pt2(rect.right(), rect.top()),
    );
    let mouse_delta_pos = mouse_pos - model.last_pos;

    // println!("Velocity: {mouse_delta_pos}");

    // println!("mousePos: {:?}", mouse_pos);

    // -----------------
    // testing with vec of BAlls
    for ball in model.balls.iter_mut() {
        if mouse_pressed == true && ball.position.distance(mouse_pos) < (ball.size) {
            // model.xy = mouse_pos;
            ball.left_pressed = true;
        }
        if ball.left_pressed == true {
            ball.position = mouse_pos;
        }
        if mouse_pressed == false && ball.left_pressed == true {
            ball.left_pressed = false;
            ball.velocity = mouse_delta_pos;
        }
        ball.position += ball.velocity;

        // Bounce of screen sides
        if (ball.position.x > rect.right()) || (ball.position.x < rect.left()) {
            ball.velocity.x *= -1.0;
        }
        if (ball.position.y > rect.top()) || (ball.position.y < rect.bottom()) {
            ball.velocity.y *= -1.0;
        }
        // mby with slider adjust velocity increase also to make the whole thing look
        // And mby also I could build this into an slider that one can use on top of the renderer
        ball.velocity *= 1.0;
        model.last_pos = mouse_pos;

        if mouse_pressed_right == true {
            ball.position = pt2(0.0, 0.0);
        }
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

    // draw.ellipse().xy(model.ball.xy).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
