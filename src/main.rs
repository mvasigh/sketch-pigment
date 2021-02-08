use nannou::noise::NoiseFn;
use nannou::{noise::Perlin, prelude::*};

struct Line {
    points: Vec<Point2<f64>>,
}

impl Line {
    fn new(y: f64, width: f64) -> Line {
        let _width = width.to_i32().unwrap();
        let mut points = Vec::new();

        for i in -_width + 50.._width - 50 {
            let x = i.to_f64().unwrap();
            points.push(Point2::new(x, y))
        }

        Line { points }
    }

    fn update<F>(&mut self, func: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        for point in self.points.iter_mut() {
            point.y = func(point.x, point.y);
        }
    }

    fn draw(&self, _app: &App, draw: &Draw, perlin: &Perlin, offset: f64) {
        let points = self.points.iter().map(|point| {
            let noise_val = perlin.get([point.x * 0.1, point.y * 0.1, offset]);
            let alpha = map_range(noise_val, -1.0, 1.0, 0.0, 1.0);
            let color = srgba(1.0, alpha.pow(2), alpha.pow(2), alpha.pow(3));

            let y = point.y + ((point.x + offset) / 4.0).sin();

            (pt2(point.x as f32, (y+ noise_val) as f32), color)
        });

        draw.polyline()
            .weight(2.0)
            .join_round()
            .points_colored(points);
    }
}

struct Model {
    _window: WindowId,
    perlin: Perlin,
    lines: Vec<Line>,
    offset: f64,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    let perlin = Perlin::new();

    let mut lines = Vec::new();
    for i in -8..9 {
        let y = i.to_f64().unwrap() * 32.0;
        lines.push(Line::new(y, 800.0));
    }

    Model {
        _window,
        perlin,
        lines,
        offset: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let offset = model.offset.to_owned();
    // for line in model.lines.iter_mut() {
    //     line.update(|x, y| {
    //         let base = (x + offset).sin();

    //         // println!("{}", base);
    //         let new_val = y + base;

    //         new_val
    //     });
    // }

    model.offset += 0.009;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.rect().w_h(800.0, 800.0).rgba(0.0, 0.0, 0.0, 0.1);

    for line in model.lines.iter() {
        line.draw(&app, &draw, &model.perlin, model.offset.to_owned());
    }

    draw.to_frame(app, &frame).unwrap();
}
