use std::cmp::{max, min};
use std::ops::{Add, AddAssign, Div, Mul};
use std::time::Duration;

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

struct Grid {
    buffer: [[u8; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    fn display(&self, draw: &Draw, bounds: &Rect) {
        let cell_wh = bounds.wh().div(Vec2::new(GRID_WIDTH as f32, GRID_HEIGHT as f32));
        let grid_start = bounds.top_left().add(cell_wh.mul(Vec2::new(0.5, -0.5)));

        for i in 0..self.buffer.len() {
            let row = self.buffer[i];
            for j in 0..row.len() {
                let cell = row[j];
                let cell_xy = grid_start.add(cell_wh.mul(Vec2::new(j as f32, -(i as f32))));
                let r = Rect::from_xy_wh(cell_xy, cell_wh).pad(1.0);
                draw.rect()
                    .xy(r.xy())
                    .wh(r.wh())
                    .rgb8(255 - cell.wrapping_mul(64), 255, 128);
            }
        }
    }
}

struct Model {
    fps: f64,
    time_past: Duration,
    counter: usize,
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    Model {
        fps: 0.0,
        time_past: Duration::ZERO,
        counter: 0,
        grid: Grid { buffer: [[1; GRID_WIDTH]; GRID_HEIGHT] },
    }
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {

}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.fps = (1000.0).div(update.since_last.as_millis() as f64);
    // model.time_past.add_assign(update.since_last);

    // if model.time_past.as_millis() >= 0 {
        let i = model.counter;
        let row = (i / GRID_WIDTH) % GRID_HEIGHT;
        let column = i % GRID_WIDTH;
        let color = (i / (GRID_HEIGHT * GRID_WIDTH) + 2) % 255;
        model.grid.buffer[row][column] = color as u8;

        model.counter += 1;
        // model.time_past = Duration::ZERO;
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.grid.display(&draw, &app.window_rect());

    draw_fps(app, &draw, model);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_fps(app: &App, draw: &Draw, model: &Model) {
    let text_x_y = app.main_window().rect().top_left().add(Point2::new(50.0, -50.0));
    let text = format!("{:.2} FPS", model.fps);

    draw.text(&text)
        .xy(text_x_y)
        .color(BLACK)
        .font_size(18);
}
