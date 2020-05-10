use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  x: f32,
  y: f32,
  xspeed: f32,
  yspeed: f32,
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();

  Model {
    x: 0.0,
    y: 0.0,
    xspeed: 1.0,
    yspeed: 3.3,
  }
}

fn update(app: &App, model: &mut Model, _update: Update) {
  let win_rect = app.window_rect();

  model.x = model.x + model.xspeed;
  model.y = model.y + model.yspeed;

  if (model.x > win_rect.right()) || (model.x < win_rect.left()) {
    model.xspeed = model.xspeed * -1.0;
  }

  if (model.y > win_rect.top()) || (model.y < win_rect.bottom()) {
    model.yspeed = model.yspeed * -1.0;
  }
}

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(WHITE);

  draw
    .ellipse()
    .x_y(model.x, model.y)
    .w_h(45.0, 45.0)
    .stroke(BLACK)
    .stroke_weight(2.0)
    .rgb8(177, 177, 177);

  draw.to_frame(app, &frame).unwrap();
}
