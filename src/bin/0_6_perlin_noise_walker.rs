extern crate nannou;
extern crate perlin_noise;
extern crate rand;

use nannou::prelude::*;
use perlin_noise::PerlinNoise;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  walker: Walker,
}

struct Walker {
  x: f32,
  y: f32,
  tx: f32,
  ty: f32,
}

impl Walker {
  fn new() -> Walker {
    Walker {
      x: 0.0,
      y: 0.0,
      tx: random_range(0.0, 1000.0),
      ty: random_range(0.0, 1000.0),
    }
  }

  fn step(&mut self, w: f32, h: f32) {
    let noise = PerlinNoise::new();
    let xn = noise.get(self.tx as f64) as f32;
    let yn = noise.get(self.ty as f64) as f32;
    let window_w_half = w / 2.0;
    let window_h_half = h / 2.0;

    let mapped_x = map_range(xn, -0.1, 1.1, -window_w_half, window_w_half);
    let mapped_y = map_range(yn, -0.1, 1.1, -window_h_half, window_h_half);

    self.x = mapped_x;
    self.y = mapped_y;
    self.tx += 0.000005;
    self.ty += 0.000005;
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Model {
    walker: Walker::new(),
  }
}

fn update(app: &App, Model { walker }: &mut Model, _update: Update) {
  let (width, height) = app.window_rect().w_h();
  walker.step(width, height);
}

fn view(app: &App, Model { walker }: &Model, frame: Frame) {
  let draw = app.draw();

  if app.elapsed_frames() == 1 {
    draw.background().color(WHITE);
  }

  draw
    .ellipse()
    .x_y(walker.x, walker.y)
    .w_h(48.0, 48.0)
    .color(GREY)
    .stroke_weight(2.0)
    .stroke(BLACK);

  draw.to_frame(app, &frame).unwrap();
}
