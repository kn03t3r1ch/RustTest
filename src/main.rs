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
    // mby with slider adjust velocity increase also to make the whole thing look
    // And mby also I could build this into an slider that one can use on top of the renderer
    model.ball.velocity *= 0.995;
    model.last_pos = mouse_pos;

    if mouse_pressed_right == true {
        model.ball.xy = pt2(0.0, 0.0);
    }

    // println!("mousePos: {:?}", mouse_pos);

    // -----------------
    // testing with vec of BAlls
    for thing in model.things.iter_mut() {
        if mouse_pressed == true && thing.position.distance(mouse_pos) < (thing.size * 2.0) {
            // model.xy = mouse_pos;
            thing.left_pressed = true;
        }
        if thing.left_pressed == true {
            thing.position = mouse_pos;
        }
        if mouse_pressed == false && thing.left_pressed == true {
            thing.left_pressed = false;
            thing.velocity = mouse_delta_pos;
        }

        thing.position += thing.velocity;

        // Bounce of screen sides
        if (thing.position.x > rect.right()) || (thing.position.x < rect.left()) {
            thing.velocity.x *= -1.0;
        }
        if (thing.position.y > rect.top()) || (thing.position.y < rect.bottom()) {
            thing.velocity.y *= -1.0;
        }
        // mby with slider adjust velocity increase also to make the whole thing look
        // And mby also I could build this into an slider that one can use on top of the renderer
        thing.velocity *= 0.995;
        model.last_pos = mouse_pos;

        if mouse_pressed_right == true {
            thing.position = pt2(0.0, 0.0);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(1.0, 0.5, 0.5, 0.03);

    for thing in model.things.iter() {
        draw.ellipse()
            .xy(thing.position)
            .radius(thing.size)
            .color(thing.color);
    }

    draw.ellipse().xy(model.ball.xy).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
