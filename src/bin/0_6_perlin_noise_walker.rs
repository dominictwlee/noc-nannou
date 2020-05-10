use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  walker: Walker,
}

struct Walker {
  x: f32,
  y: f32,
  noise_x: Perlin,
  noise_y: Perlin,
  tx: f32,
  ty: f32,
}

impl Walker {
  fn new() -> Walker {
    Walker {
      x: 0.0,
      y: 0.0,
      noise_x: Perlin::new(),
      noise_y: Perlin::new(),
      tx: random_range(0.0, 1000.0),
      ty: random_range(0.0, 1000.0),
    }
  }

  fn step(&mut self, w: f32, h: f32) {
    let xn = self.noise_x.get([self.tx as f64, 1.0]) as f32;
    let yn = self.noise_y.get([self.ty as f64, 1.0]) as f32;
    let w_boundary = w / 2.0;
    let h_boundary = h / 2.0;

    let mapped_x = map_range(xn, -0.02, 1.0, -100.0, w_boundary);
    let mapped_y = map_range(yn, -0.02, 1.0, -80.0, h_boundary);

    self.x = mapped_x;
    self.y = mapped_y;
    self.tx += 0.01;
    self.ty += 0.01;
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
