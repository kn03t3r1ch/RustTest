use nannou::{glam::Vec2Swizzles, lyon::math::rect, prelude::*, state::keys::Down};
use rand;

struct Model {}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {}
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let poly_width = 50;
    let t = app.time;
    let real_amp = 0.5;
    let mouse_pressed = app.mouse.buttons.left().is_down();

    if mouse_pressed == true {
        draw.ellipse()
            .x_y(app.mouse.x, app.mouse.y)
            .w_h(50.0, 50.0)
            .color(WHITE)
            .stroke(BLACK);
    }

    let points = (0..poly_width).map(|i| {
        let x = map_range(i, 0, poly_width - 1, win.left(), win.right());
        let fract = i as f32 / poly_width as f32;
        let amp = (t + fract * TAU * 3.0).sin();
        let y = map_range(
            amp,
            -1.0,
            1.0,
            win.bottom() * real_amp,
            win.top() * real_amp,
        );
        let rgb = hsla(fract, 1.0, 0.5, 1.0);
        let point = pt2(x, y);
        (point, rgb)
    });

    // I wanted to have this as different kinds of weights of the polyline
    // but my guess why it isn't working is that this is a kind of array but the
    // mehod of weight is only capable of having one weight for all parts of
    // the polyline
    let thickness = (0..poly_width).map(|i| {
        let rand_thick = rand::random::<f32>();
        rand_thick
    });

    draw.background().color(BLACK);

    draw.polyline().weight(3.0).points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
