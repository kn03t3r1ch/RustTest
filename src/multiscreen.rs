use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    a: WindowId,
    b: WindowId,
    c: WindowId,
}

fn model(app: &App) -> Model {
    let a = app
        .new_window()
        .title("window a")
        .event(event_a)
        .build()
        .unwrap();

    let b = app
        .new_window()
        .title("window b")
        .event(event_b)
        .build()
        .unwrap();

    let c = app
        .new_window()
        .title("window a")
        .event(event_c)
        .build()
        .unwrap();
    Model { a, b, c }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event_a(_app: &App, _model: &mut Model, event: WindowEvent) {
    println!("Window a: {:?}", event);
}

fn event_b(_app: &App, _model: &mut Model, event: WindowEvent) {
    println!("Window b: {:?}", event);
}

fn event_c(_app: &App, _model: &mut Model, event: WindowEvent) {
    println!("Window c: {:?}", event);
}

fn view(_app: &App, model: &Model, frame: Frame) {
    match frame.window_id() {
        id if id == model.a => frame.clear(GREEN),
        id if id == model.b => frame.clear(BLUE),
        id if id == model.c => frame.clear(RED),
        _ => (),
    }
}
