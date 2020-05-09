extern crate rand;
use nannou::prelude::*;
use rand::distributions::{Distribution, Normal};

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  x: f32,
}

impl Model {
  fn distribute(&mut self) {
    let normal = Normal::new(0.0, 60.0);
    let v = normal.sample(&mut rand::thread_rng());
    self.x = v as f32;
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Model { x: 0 as f32 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
  model.distribute();
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  if app.elapsed_frames() == 1 {
    draw.background().color(WHITE);
  }

  draw
    .ellipse()
    .x_y(model.x, 0.0)
    .w_h(16.0, 16.0)
    .rgba(0.0, 0.0, 0.0, 0.2);

  draw.to_frame(app, &frame).unwrap();
}
