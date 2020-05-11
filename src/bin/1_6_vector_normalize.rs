use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  mouse: Vector2,
  anchor: Vector2,
}

impl Model {
  fn new() -> Model {
    Model {
      mouse: vec2(0.0, 0.0),
      anchor: vec2(0.0, 0.0),
    }
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Model::new()
}

fn update(app: &App, model: &mut Model, _update: Update) {
  let mouse_position = app.mouse.position();
  model.mouse = (mouse_position - model.anchor).normalize() * 150.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(WHITE);

  draw
    .line()
    .weight(2.0)
    .color(BLACK)
    .points(model.anchor, model.mouse);

  draw.to_frame(app, &frame).unwrap();
}
