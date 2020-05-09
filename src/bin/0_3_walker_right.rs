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
    let r = random_f32();

    if r < 0.4 {
      self.x += 1;
    } else if r < 0.6 {
      self.x -= 1;
    } else if r < 0.8 {
      self.y += 1;
    } else {
      self.y -= 1;
    }
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
  let window_rect = app.window_rect();
  let window_width = window_rect.w();

  if app.elapsed_frames() == 1 {
    draw.background().color(WHITE);
  }

  draw
    .ellipse()
    .x_y(
      (walker.x as f32).min(window_width / 2.0 - 5.0),
      walker.y as f32,
    )
    .w_h(2.0, 2.0)
    .rgba(0.0, 0.0, 0.0, 1.0);

  draw.to_frame(app, &frame).unwrap();
}
