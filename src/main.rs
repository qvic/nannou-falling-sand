use std::cmp::{max, min};
use std::ops::{Add, Div};

use nannou::prelude::*;
use nannou::state::Mouse;

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

struct DoubleBuffer {
    buffer: Vec<Vec<[u8; 2]>>,
    current: usize,
    fill: u8,
}

impl DoubleBuffer {
    fn new(width: usize, height: usize, fill: u8) -> Self {
        let v = vec![vec![[fill, fill]; width]; height];
        DoubleBuffer { buffer: v, current: 0, fill }
    }

    fn set(&mut self, row: usize, column: usize, value: u8) {
        self.buffer[row][column][self.current] = value;
    }

    fn get(&self, row: usize, column: usize) -> u8 {
        self.buffer[row][column][self.current]
    }

    fn get_old(&self, row: usize, column: usize) -> u8 {
        self.buffer[row][column][1 - self.current]
    }

    fn switch(&mut self) {
        let next = 1 - self.current;

        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                cell[next] = self.fill;
            }
        }

        self.current = next;
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
                        .w_h(cell_w, cell_h)
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
    spawn: bool,
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .resizable(false)
        .clear_color(WHITE)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .view(view)
        .build()
        .unwrap();

    let mut model = Model {
        fps: 0.0,
        counter: 0,
        spawn: false,
        grid: Grid {
            buffer: DoubleBuffer::new(GRID_WIDTH, GRID_HEIGHT, 0),
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
        },
    };

    // for col in 0..GRID_WIDTH {
    //     model.grid.buffer.set(0, col, 1);
    // }

    model
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    model.spawn = true;
    spawn_cell(app, model, Point2::new(app.mouse.x, app.mouse.y));
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.spawn = false;
}

fn mouse_moved(app: &App, model: &mut Model, point: Point2) {
    if model.spawn {
        spawn_cell(app, model, point);
    }
}

fn spawn_cell(app: &App, model: &mut Model, point: Point2) {
    let bounds = app.window_rect();
    let pixels_per_row = bounds.h() / GRID_HEIGHT as f32;
    let pixels_per_column = bounds.w() / GRID_WIDTH as f32;
    let grid_row = (bounds.h() / 2.0 - point.y) / pixels_per_row;
    let grid_column = (point.x + bounds.w() / 2.0) / pixels_per_column;

    let grid_row_truncated = max(0, min(grid_row as usize, GRID_HEIGHT - 1));
    let grid_column_truncated = max(0, min(grid_column as usize, GRID_WIDTH - 1));

    model.grid.buffer.set(grid_row_truncated, grid_column_truncated, 1);
    // println!("{:?} {:?}", grid_row, grid_column);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.fps = (1000.0).div(update.since_last.as_millis() as f64);

    model.grid.buffer.switch();

    update_grid(&mut model.grid);

    model.counter += 1;
}

fn update_grid(grid: &mut Grid) {
    for row in 0..grid.height {
        for column in 0..grid.width {
            let current_value = grid.buffer.get_old(row, column);

            if current_value > 0 {
                if row < grid.height - 1 && grid.buffer.get_old(row + 1, column) == 0 {
                    grid.buffer.set(row + 1, column, current_value);
                } else {
                    grid.buffer.set(row, column, current_value);
                }
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // frame.clear(WHITE);
    let draw = app.draw();
    // draw.background().color(WHITE);

    model.grid.display(&draw, &app.window_rect());

    // print_fps(model);

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
