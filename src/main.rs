use crate::model::ball::Ball;
use crate::model::Model;
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
    println!("event_a {:?}", model);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse_pressed = app.mouse.buttons.left().is_down();
    let mouse_pos = app.mouse.position();
    if mouse_pressed == true && model.ball.xy.distance(mouse_pos) < (model.ball.ball_size) {
        // model.xy = mouse_pos;
        model.ball.left_pressed = true;
    }
    if model.ball.left_pressed == true {
        model.ball.xy = mouse_pos;
    }
    if mouse_pressed == false {
        model.ball.left_pressed = false;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().xy(model.ball.xy).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
