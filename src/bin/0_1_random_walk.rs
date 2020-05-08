extern crate rand;
use nannou::prelude::*;
use rand::Rng;

fn main() {
  nannou::app(model).update(update).run();
}

struct Walker {
  x: i32,
  y: i32,
  has_background: bool,
}

fn model(app: &App) -> Walker {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Walker {
    x: 0,
    y: 0,
    has_background: true,
  }
}

fn update(_app: &App, model: &mut Walker, _update: Update) {
  let choice = rand::thread_rng().gen_range(0, 4);

  match choice {
    0 => model.x += 1,
    1 => model.x -= 1,
    2 => model.y += 1,
    3 => model.y -= 1,
    _ => (),
  }

  model.has_background = true;
}

fn view(app: &App, model: &Walker, frame: Frame) {
  let draw = app.draw();

  if app.elapsed_frames() == 1 {
    draw.background().color(WHITE);
  }

  draw
    .ellipse()
    .x_y(model.x as f32, model.y as f32)
    .w_h(2.0, 2.0)
    .rgba(0.0, 0.0, 0.0, 1.0);

  draw.to_frame(app, &frame).unwrap();
}
