// This is no longer needed because we imported Ball already within the Model crate
// use crate::model::ball::Ball;
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
    //println!("Velocity: {mouse_delta_pos}");

    if mouse_pressed == true && model.ball.xy.distance(mouse_pos) < (model.ball.size * 2.0) {
        // model.xy = mouse_pos;
        model.ball.left_pressed = true;
    }
    if model.ball.left_pressed == true {
        model.ball.xy = mouse_pos;
    }
    if mouse_pressed == false && model.ball.left_pressed == true {
        model.ball.left_pressed = false;
        model.ball.velocity = mouse_delta_pos;
    }

    model.ball.xy += model.ball.velocity;

    // Bounce of screen sides
    if (model.ball.xy.x > rect.right()) || (model.ball.xy.x < rect.left()) {
        model.ball.velocity.x *= -1.0;
    }
    if (model.ball.xy.y > rect.top()) || (model.ball.xy.y < rect.bottom()) {
        model.ball.velocity.y *= -1.0;
    }
    // Clamp at screen width and height
    // There is still a problem -> the ball clamps at the edge of the window but if I let go
    // of the mouse outside of the renderer then the ball vanishes at the border of the window
    // if (model.ball.xy.x > rect.right()) && (mouse_pressed == true) {
    //     model.ball.xy.x = rect.right();
    // }
    // if (model.ball.xy.x < rect.left()) && (mouse_pressed == true) {
    //     model.ball.xy.x = rect.left();
    // }
    // if (model.ball.xy.y > rect.top()) && (mouse_pressed == true) {
    //     model.ball.xy.y = rect.top();
    // }
    // if (model.ball.xy.y < rect.bottom()) && (mouse_pressed == true) {
    //     model.ball.xy.y = rect.bottom();
    // }

    // if (mouse_pos.x > rect.right()) && (mouse_pressed == true) {
    //     mouse_pos.x = rect.right() - model.ball.size;
    // }
    // if (mouse_pos.x < rect.left()) && (mouse_pressed == true) {
    //     mouse_pos.x = rect.left() - model.ball.size;
    // }
    // if (mouse_pos.y > rect.top()) && (mouse_pressed == true) {
    //     mouse_pos.y = rect.top() - model.ball.size;
    // }
    // if (mouse_pos.y < rect.bottom()) && (mouse_pressed == true) {
    //     mouse_pos.y = rect.bottom() - model.ball.size;
    // }

    // mby with slider adjust velocity increase also to make the whole thing look
    // And mby also I could build this into an slider that one can use on top of the renderer
    model.ball.velocity *= 0.995;
    model.last_pos = mouse_pos;

    if mouse_pressed_right == true {
        model.ball.xy = pt2(0.0, 0.0);
    }

    println!("mousePos: {:?}", mouse_pos);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(1.0, 1.0, 1.0, 0.03);

    draw.ellipse().xy(model.ball.xy).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
