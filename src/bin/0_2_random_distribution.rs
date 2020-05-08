extern crate rand;
use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  counts: Vec<i32>,
}

const TOTAL: i32 = 20;

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  let mut counts = Vec::new();

  for _ in 0..TOTAL {
    counts.push(0);
  }

  Model { counts }
}

fn update(_app: &App, Model { counts }: &mut Model, _update: Update) {
  let index = random_range(0, TOTAL);
  counts[index as usize] += 1;
}

fn view(app: &App, Model { counts }: &Model, frame: Frame) {
  let draw = app.draw();
  let win = app.window_rect();
  let win_width = win.len();
  let rect_width = win_width / (TOTAL as f32);
  let rect_mid_point = rect_width / 2.0;

  draw.background().color(WHITE);

  for (i, &count) in counts.iter().enumerate() {
    let max_left = win.left();
    let max_bottom = win.bottom();
    let offset_x = (i as f32) * rect_width;

    draw
      .rect()
      .x_y(max_left + rect_mid_point + offset_x, max_bottom)
      .w_h(rect_width, count as f32)
      .color(GREY)
      .stroke_weight(2.0)
      .stroke(BLACK);
  }

  draw.to_frame(app, &frame).unwrap();
}
