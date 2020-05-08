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
    let choice = random_range(0, 4);

    match choice {
      0 => self.x += 1,
      1 => self.x -= 1,
      2 => self.y += 1,
      3 => self.y -= 1,
      _ => (),
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
