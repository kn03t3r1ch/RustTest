use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    xy: Vec2,
    ball_size: f32,
}

fn model(app: &App) -> Model {
    let xy = pt2(0.0, 0.0);
    let ball_size = 25.0;
    let _window = app.new_window().view(view).build().unwrap();

    Model { xy, ball_size }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse_pressed = app.mouse.buttons.left().is_down();
    let mouse_pos = app.mouse.position();
    if mouse_pressed == true && model.xy.distance(mouse_pos) < (model.ball_size) {
        model.xy = mouse_pos;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().xy(model.xy).color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
