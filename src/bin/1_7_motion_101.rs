use nannou::prelude::*;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  mover: Mover,
}

struct Mover {
  location: Point2,
  velocity: Vector2,
}

impl Mover {
  fn new(win: Rect) -> Self {
    let (l, r, b, t) = win.l_r_b_t();
    let location = pt2(random_range(l, r), random_range(b, t));
    let velocity = vec2(random_range(-2.0, 2.0), random_range(-2.0, 2.0));
    Mover { location, velocity }
  }

  fn update(&mut self) {
    self.location += self.velocity;
  }

  fn display(&self, draw: &Draw) {
    draw
      .ellipse()
      .xy(self.location)
      .w_h(48.0, 48.0)
      .gray(0.5)
      .stroke_weight(2.0)
      .stroke(BLACK);
  }

  fn check_edges(&mut self, win: Rect) {
    let (l, r, b, t) = win.l_r_b_t();

    if self.location.x > r {
      self.location.x = l;
    } else if self.location.x < l {
      self.location.x = r;
    }

    if self.location.y > t {
      self.location.y = b;
    } else if self.location.y < b {
      self.location.y = t;
    }
  }
}

fn model(app: &App) -> Model {
  app.new_window().size(640, 360).view(view).build().unwrap();
  let mover = Mover::new(app.window_rect());
  Model { mover }
}

fn update(app: &App, Model { mover }: &mut Model, _update: Update) {
  let win = app.window_rect();
  mover.update();
  mover.check_edges(win);
}

fn view(app: &App, Model { mover }: &Model, frame: Frame) {
  let draw = app.draw();
  draw.background().rgba(1.0, 1.0, 1.0, 0.03);
  mover.display(&draw);
  draw.to_frame(app, &frame).unwrap();
}
