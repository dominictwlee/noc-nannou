use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  mouse: Vector2,
  anchor: Vector2,
  perlin: Perlin,
  xoff: f64,
  yoff: f64,
}

impl Model {
  fn new() -> Model {
    Model {
      mouse: vec2(0.0, 0.0),
      anchor: vec2(0.0, 0.0),
      perlin: Perlin::new(),
      xoff: 0.0,
      yoff: 1000.0,
    }
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Model::new()
}

fn update(app: &App, model: &mut Model, _update: Update) {
  let mouse_position = app.mouse.position();
  let noise_x = model.perlin.get([model.xoff, 0.0]);
  let noise_y = model.perlin.get([model.yoff, 0.0]);
  let mapped_noise_x = map_range(noise_x, 0.0, 1.0, -50.0, 50.0);
  let mapped_noise_y = map_range(noise_y, 0.0, 1.0, -50.0, 50.0);

  model.anchor = vec2(mapped_noise_x, mapped_noise_y);
  model.mouse = mouse_position - model.anchor;
  model.xoff += 0.01;
  model.yoff += 0.01;
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
