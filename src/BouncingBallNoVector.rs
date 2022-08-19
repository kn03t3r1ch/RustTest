use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
    ball_size: f32,
}

fn model(app: &App) -> Model {
    let x = 100.0;
    let y = 100.0;
    let x_speed = 2.5;
    let y_speed = 2.0;
    let ball_size = 50.0;

    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    Model {
        x,
        y,
        x_speed,
        y_speed,
        ball_size,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.x = model.x + model.x_speed;
    model.y = model.y + model.y_speed;

    let win_rect = app.window_rect();
    let screen_ball_adjust = model.ball_size / 2.0;

    if (model.x > win_rect.right() - screen_ball_adjust)
        || (model.x < win_rect.left() + screen_ball_adjust)
    {
        model.x_speed = model.x_speed * -1.0;
    }
    if (model.y > win_rect.top() - screen_ball_adjust)
        || (model.y < win_rect.bottom() + screen_ball_adjust)
    {
        model.y_speed = model.y_speed * -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(model.ball_size, model.ball_size)
        .color(GREY)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
