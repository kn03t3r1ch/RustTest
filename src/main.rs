// This is no longer needed because we imported Ball already within the Model crate
// use crate::model::ball::Ball;
use crate::model::Model;
use model::ball::Ball;
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
                println!("intersection at: {:?}", model.balls[j].position);
                model.balls[i].color = hsl(1.0, 0.0, 1.0);
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
        model.balls[i].velocity *= 1.0;
        model.last_pos = mouse_pos;
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
