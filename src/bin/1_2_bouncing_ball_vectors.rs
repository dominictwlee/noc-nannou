use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  ball: Ball,
}

struct Ball {
  location: Point2,
  velocity: Vector2,
}

impl Ball {
  fn new() -> Ball {
    let location = pt2(100.0, 100.0);
    let velocity = vec2(2.5, 5.0);

    Ball { location, velocity }
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();
  let ball = Ball::new();

  Model { ball }
}

fn update(app: &App, Model { ball }: &mut Model, _update: Update) {
  let win_rect = app.window_rect();

  ball.location += ball.velocity;

  if (ball.location.x > win_rect.right()) || (ball.location.x < win_rect.left()) {
    ball.velocity.x *= -1.0;
  }

  if (ball.location.y > win_rect.top()) || (ball.location.y < win_rect.bottom()) {
    ball.velocity.y *= -1.0;
  }
}

fn view(app: &App, Model { ball }: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(WHITE);

  draw
    .ellipse()
    .x_y(ball.location.x, ball.location.y)
    .w_h(45.0, 45.0)
    .stroke(BLACK)
    .stroke_weight(2.0)
    .rgb8(177, 177, 177);

  draw.to_frame(app, &frame).unwrap();
}
