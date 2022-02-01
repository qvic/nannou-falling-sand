use std::cmp::min;
use std::ops::{Add, Div};

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const COLORS: [[u8; 3]; 5] = [
    [255, 255, 255],
    [49, 53, 82],
    [184, 64, 94],
    [46, 176, 134],
    [238, 230, 206],
];

struct Grid {
    buffer: DoubleBuffer,
    width: usize,
    height: usize,
}

type Matrix = Vec<Vec<u8>>;

struct DoubleBuffer {
    buffers: [Matrix; 2],
    current: usize,
}

impl DoubleBuffer {
    fn new(width: usize, height: usize, fill: u8) -> Self {
        let v0 = vec![vec![fill; width]; height];
        let v1 = vec![vec![fill; width]; height];
        DoubleBuffer { buffers: [v0, v1], current: 0 }
    }

    fn set(&mut self, row: usize, column: usize, value: u8) {
        self.buffers[self.current][row][column] = value;
    }

    fn get(&self, row: usize, column: usize) -> u8 {
        self.buffers[self.current][row][column]
    }

    fn get_old(&self, row: usize, column: usize) -> u8 {
        self.buffers[1 - self.current][row][column]
    }

    fn switch(&mut self) {
        self.current = 1 - self.current;
    }
}

impl Grid {
    fn display(&self, draw: &Draw, bounds: &Rect) {
        let cell_w = bounds.w() / GRID_WIDTH as f32;
        let cell_h = bounds.h() / GRID_HEIGHT as f32;
        let start_x = bounds.left() + cell_w / 2.0;
        let start_y = bounds.top() - cell_h / 2.0;

        for row in 0..self.height {
            let row_y = start_y - row as f32 * cell_h;

            for column in 0..self.width {
                let value = self.buffer.get(row, column);
                let old_value = self.buffer.get_old(row, column);


                if value != old_value {
                    let color = COLORS[value as usize];
                    let cell_x = start_x + column as f32 * cell_w;
                    draw.rect()
                        .x_y(cell_x, row_y)
                        .w_h(cell_w - 1.0, cell_h - 1.0)
                        .rgb8(color[0], color[1], color[2]);
                }
            }
        }
        // println!("{:?}", self.buffer.buffers[self.buffer.current]);
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
        .resizable(false)
        .clear_color(WHITE)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();
    // app.draw().background().color(WHITE);

    let mut model = Model {
        fps: 0.0,
        counter: 0,
        grid: Grid {
            buffer: DoubleBuffer::new(GRID_WIDTH, GRID_HEIGHT, 0),
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
        },
    };

    model.grid.buffer.set(0, 1, 1);

    model
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {

}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.fps = (1000.0).div(update.since_last.as_millis() as f64);

    model.grid.buffer.switch();

    update_grid(&mut model.grid, model.counter);

    model.counter += 1;
}

fn update_grid(grid: &mut Grid, counter: usize) {
    for row in 0..grid.height {
        for column in 0..grid.width {
            let current_value = grid.buffer.get_old(row, column);

            let is_still = current_value > 0 && row == grid.height - 1;
            let has_cell_top = if row > 0 { grid.buffer.get_old(row - 1, column) > 0 } else { false };
            let remain = is_still || has_cell_top;

            grid.buffer.set(row, column, remain as u8);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // frame.clear(WHITE);
    let draw = app.draw();
    // draw.background().color(WHITE);

    model.grid.display(&draw, &app.window_rect());

    print_fps(model);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_fps(app: &App, draw: &Draw, model: &Model) {
    let text_x_y = app.main_window().rect().top_left().add(Point2::new(50.0, -30.0));
    let text = format!("{:.2} FPS", model.fps);

    draw.text(&text)
        .xy(text_x_y)
        .color(BLACK)
        .font_size(18);
}

fn print_fps(model: &Model) {
    println!("{:.3}", model.fps);
}
