extern crate rand;
use nannou::prelude::*;
use nannou::state::Mouse;

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

  fn step_mouse(&mut self, mouse_x: f32, mouse_y: f32) {
    let r = random_f32();

    if r < 0.5 {
      self.x += mouse_x.signum() as i32;
      self.y += mouse_y.signum() as i32;
    } else {
      self.x += random_range(-1, 2);
      self.y += random_range(-1, 2);
    }
  }

  fn _step(&mut self) {
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

fn update(app: &App, Model { walker }: &mut Model, _update: Update) {
  let Mouse { x, y, .. } = app.mouse;
  walker.step_mouse(x, y);
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
