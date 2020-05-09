extern crate rand;
use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  walker: Walker,
}

struct Walker {
  x: i32,
  y: i32,
}

impl Walker {
  fn new() -> Walker {
    Walker { x: 0, y: 0 }
  }

  fn step(&mut self) {
    let r = Walker::montecarlo();

    if r < 0.05 {
      self.x += random_range(-1, 1);
      self.y += random_range(-1, 1);
    } else {
      self.x += random_range(-10, 10);
      self.y += random_range(-10, 10);
    }
  }

  fn montecarlo() -> f32 {
    let mut r1 = random_f32();
    let mut p = r1.clone();
    let r2 = random_f32();

    while r2 > p {
      let r = random_f32();
      r1 = r;
      p = r;
    }

    r1
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Model {
    walker: Walker::new(),
  }
}

fn update(_app: &App, Model { walker }: &mut Model, _update: Update) {
  walker.step();
}

fn view(app: &App, Model { walker }: &Model, frame: Frame) {
  let draw = app.draw();

  if app.elapsed_frames() == 1 {
    draw.background().color(WHITE);
  }

  draw
    .ellipse()
    .x_y(walker.x as f32, walker.y as f32)
    .w_h(2.0, 2.0)
    .rgba(0.0, 0.0, 0.0, 1.0);

  draw.to_frame(app, &frame).unwrap();
}
