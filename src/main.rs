use nannou::lyon;
use nannou::lyon::algorithms::walk::{walk_along_path, RegularPattern};
use nannou::lyon::path::iterator::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Message {
    text: String,
}

impl Message {
    fn new(text: String) -> Self {
        Message { text }
    }
}

#[derive(Debug, Clone)]
struct Kazaguruma {
    points: Vec<Point2>,
    angle: f32,
    acceleration: Vec2,
    velocity: Vec2,
    max_list_length: usize,
    list_full: bool,
    list_empty: bool,
    color: Rgba<f32>,
}

impl Kazaguruma {
    fn new(win_rect: Rect) -> Self {
        let start_point = pt2(
            random_range(win_rect.left() + 100.0, win_rect.right() - 100.0),
            random_range(win_rect.bottom() + 200.0, win_rect.top() - 200.0),
        );
        let points = vec![start_point];
        let angle = random_range(0.0, 360.0);
        let acceleration = vec2(0.0, 0.0);
        let velocity = start_point;
        let max_list_length = random_range(25, 50);
        let list_full = false;
        let list_empty = false;
        let color = rgba(242.0 / 255.0, 5.0 / 255.0, 25.0 / 255.0, 10.0 / 255.0);

        Kazaguruma {
            points,
            angle,
            acceleration,
            velocity,
            max_list_length,
            list_full,
            list_empty,
            color,
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    fn force(&self) -> Vec2 {
        let force_x = random_range(-20.0, 60.0);
        let force_y = random_range(-20.0, 60.0);
        let length = random_range(25.0, 50.0);

        let normalized = vec2(force_x, force_y).normalize();

        normalized * length
    }

    fn check_if_points_exist(&mut self) {
        if self.points.is_empty() {
            self.list_empty = true;
        }
    }

    fn check_if_the_list_is_full(&mut self) {
        if self.max_list_length == self.points.len() {
            self.list_full = true;
        }
    }

    fn angle_update(&mut self) {
        if self.angle == 360.0 {
            self.angle = 0.0;
        }
        self.angle += 2.0;
    }

    fn points_update(&mut self, win_rect: Rect) {
        if !self.list_empty {
            self.check_if_the_list_is_full();

            if !self.list_full {
                let force = self.force();
                self.apply_force(force);
                self.velocity += self.acceleration;

                let new_point = self.velocity;
                self.points.push(new_point);
                self.acceleration *= 0.0;
            } else {
                self.points.remove(0);
                if !self.points.is_empty() {
                    self.points.remove(0);
                }
                if self.points.is_empty() {
                    let start_point = pt2(
                        random_range(win_rect.left() + 100.0, win_rect.right() - 100.0),
                        random_range(win_rect.bottom() + 200.0, win_rect.top() - 200.0),
                    );
                    self.points.push(start_point);
                    self.velocity = start_point;
                    self.angle = random_range(0.0, 360.0);
                    self.acceleration *= 0.0;
                    self.list_empty = false;
                    self.list_full = false;
                }
            }
        }
    }
}

struct Model {
    message: Message,
    count: usize,
    time: u64,
    kazaguruma_list: Vec<Kazaguruma>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(720, 1280).view(view).build().unwrap();

    let message = Message::new("Be yourself.".to_string());
    let time = 0;
    let count = 0;

    let kazaguruma_list = vec![Kazaguruma::new(app.window_rect())];

    Model {
        message,
        time,
        count,
        kazaguruma_list,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let frame = app.elapsed_frames();
    let path = format!("./screenshots/{}.png", frame);
    screenshot(app, frame, path);

    let time = app.elapsed_frames();
    if model.time + 80 == time {
        match model.count {
            0 => {
                let message = Message::new("Stay true to yourself.".to_string());
                model.message = message;
                model.time = time;
                model.count += 1;
            }
            1 => {
                let message = Message::new("Embrace your uniqueness.".to_string());
                model.message = message;
                model.time = time;
                model.count += 1;
            }
            2 => {
                let message = Message::new("Be yourself.".to_string());
                model.message = message;
                model.time = time;
                model.count = 0;
            }
            _ => {}
        }
    }

    if model.kazaguruma_list.len() < 5 && time % 10 == 0 {
        let kazaguruma = Kazaguruma::new(app.window_rect());
        model.kazaguruma_list.push(kazaguruma);
    }

    model.kazaguruma_list.iter_mut().for_each(|kazaguruma| {
        kazaguruma.check_if_points_exist();
        if !kazaguruma.list_empty {
            kazaguruma.angle_update();
            kazaguruma.points_update(app.window_rect());
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgba(
        13.0 / 255.0,
        13.0 / 255.0,
        13.0 / 255.0,
        200.0 / 255.0,
    ));

    let win_rect = app.main_window().rect();
    let text = text(&model.message.text)
        .font_size(128)
        .center_justify()
        // .layout(&layout)
        .build(win_rect);

    let mut builder = lyon::path::Path::builder();
    for e in text.path_events() {
        builder.path_event(e);
    }
    let path = builder.build();

    let mut path_points: Vec<lyon::path::math::Point> = Vec::new();
    let mut pattern = RegularPattern {
        callback: &mut |position, _target, _distance| {
            path_points.push(position);
            true
        },
        interval: 1.0,
    };
    let tolerance = 0.01;
    let start_offset = 0.0;
    walk_along_path(path.iter().flattened(tolerance), start_offset, &mut pattern);

    path_points.iter().enumerate().for_each(|(i, p)| {
        let l = 8.0;
        draw.line()
            .start(pt2(p.x, p.y + l))
            .end(pt2(p.x, p.y - l))
            .stroke_weight(5.0)
            .rgba(242.0 / 255.0, 5.0 / 255.0, 25.0 / 255.0, 95.0 / 255.0);

        if i % 2 == 0 {
            draw.ellipse()
                .x_y(p.x, p.y)
                .radius(map_range(
                    (i as f32 * 0.05 + app.time * 4.3).sin(),
                    -1.0,
                    1.0,
                    5.0,
                    15.0,
                ))
                .rgba(12.0 / 255.0, 242.0 / 255.0, 70.0 / 255.0, 15.0 / 255.0);
        }
    });

    model.kazaguruma_list.iter().for_each(|kazaguruma| {
        draw.polyline()
            .stroke_weight(200.0)
            .points(kazaguruma.points.clone())
            .radians(vec3(0.0, 0.0, kazaguruma.angle.to_radians()))
            .color(kazaguruma.color);
    });

    draw.to_frame(app, &frame).unwrap();
}

fn screenshot(app: &App, frame_num: u64, path: String) {
    if frame_num < 239 {
        app.main_window().capture_frame(path);
    }

    if frame_num == 240 {
        println!("{}", frame_num);
    }
}
