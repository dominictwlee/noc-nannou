extern crate nannou;
extern crate rand;

use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
  nannou::app(model).run();
}

struct Model {
  pixels: Vec<(i32, i32, u8)>,
}

impl Model {
  fn new(pixels: Vec<(i32, i32, u8)>) -> Model {
    Model { pixels }
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();
  let window = app.window_rect();
  let (width, height) = window.w_h();
  let Vector2 {
    x: start_x,
    y: start_y,
  } = window.bottom_left();
  let perlin = Perlin::new();
  let mut pixels = Vec::new();

  let mut xoff = 0.0;
  for x in (start_x as i32)..(width as i32) {
    let mut yoff = 0.0;
    for y in (start_y as i32)..(height as i32) {
      let color = map_range(perlin.get([xoff, yoff]), 0.0, 1.0, 0.0, 255.0);
      pixels.push((x, y, color as u8));
      yoff += 0.01;
    }
    xoff += 0.01;
  }

  Model::new(pixels)
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(WHITE);

  for (x, y, color) in &model.pixels {
    let clr = color.clone();
    draw
      .rect()
      .x_y(x.clone() as f32, y.clone() as f32)
      .w_h(1.0, 1.0)
      .rgb8(clr, clr, clr);
  }

  draw.to_frame(app, &frame).unwrap();
}
