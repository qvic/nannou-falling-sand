use std::ops::{Add, Div};

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 300;

struct Grid {
    buffer: [[[u8; GRID_WIDTH]; GRID_HEIGHT]; 2],
    current_buffer: usize,
}

impl Grid {
    fn display(&self, draw: &Draw, bounds: &Rect) {
        let cell_w = bounds.w() / GRID_WIDTH as f32;
        let cell_h = bounds.h() / GRID_HEIGHT as f32;
        let start_x = bounds.left() + cell_w / 2.0;
        let start_y = bounds.top() - cell_h / 2.0;

        let buffer = self.buffer[self.current_buffer];
        let old_buffer = self.buffer[1 - self.current_buffer];

        for i in 0..buffer.len() {
            let row = buffer[i];
            let row_y = start_y - i as f32 * cell_h;
            for j in 0..row.len() {
                let color = row[j];
                let old_color = old_buffer[i][j];

                if color != old_color {
                    let cell_x = start_x + j as f32 * cell_w;
                    draw.rect()
                        .x_y(cell_x, row_y)
                        .w_h(cell_w, cell_h)
                        .rgb8(255 - color.wrapping_mul(64), 255, 128);
                }
            }
        }
    }
}

struct Model {
    fps: f64,
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
        counter: 0,
        grid: Grid { buffer: [[[1; GRID_WIDTH]; GRID_HEIGHT]; 2], current_buffer: 0 },
    }
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.fps = (1000.0).div(update.since_last.as_millis() as f64);

    let i = model.counter;
    let row = (i / GRID_WIDTH) % GRID_HEIGHT;
    let column = i % GRID_WIDTH;
    let color = (i / (GRID_HEIGHT * GRID_WIDTH) + 2) % 255;

    model.grid.current_buffer = 1 - model.grid.current_buffer;
    model.grid.buffer[model.grid.current_buffer][row][column] = color as u8;

    model.counter += 1;
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
