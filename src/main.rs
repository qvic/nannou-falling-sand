use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div};

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;

const MOUSE_SPAWN_RADIUS: i32 = 2;
const BACKGROUND: [u8; 3] = [255, 237, 219];
const COLORS: [[u8; 3]; 4] = [
    BACKGROUND,
    [184, 64, 94],
    [46, 176, 134],
    [49, 53, 82],
];

struct Model {
    fps: f64,
    counter: u64,
    spawn: bool,
    spawn_color: u8,
    grid: Grid,
}

#[derive(Copy, Clone)]
struct Cell {
    value: u8,
    updated: bool,
}

struct Grid {
    buffer: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                let cell = self.buffer[i * self.width + j];
                let updated_mark = if cell.updated { "+" } else { "-" };
                write!(f, "{}{} ", updated_mark, cell.value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(width: usize, height: usize, fill: u8) -> Self {
        let buffer = vec![Cell { value: fill, updated: false }; width * height];
        Grid { buffer, width, height }
    }

    fn set(&mut self, row: usize, column: usize, value: u8) {
        let cell = &mut self.buffer[row * self.width + column];
        cell.value = value;
        cell.updated = true;
    }

    fn get(&self, row: usize, column: usize) -> &Cell {
        &self.buffer[row * self.width + column]
    }

    fn step(&mut self) {
        for cell in self.buffer.iter_mut() {
            cell.updated = false;
        }

        for row in 0..self.height {
            for column in 0..self.width {
                let current_cell = self.get(row, column);

                if !current_cell.updated {
                    let current_value = current_cell.value;

                    if current_value > 0 {
                        if row < self.height - 1 {
                            if self.get(row + 1, column).value == 0 {
                                self.set(row, column, 0);
                                self.set(row + 1, column, current_value);
                            } else if column > 0 && self.get(row + 1, column - 1).value == 0 {
                                self.set(row, column, 0);
                                self.set(row + 1, column - 1, current_value);
                            } else if column < self.width - 1 && self.get(row + 1, column + 1).value == 0 {
                                self.set(row, column, 0);
                                self.set(row + 1, column + 1, current_value);
                            }
                        }
                    }
                }
            }
        }
    }

    fn display(&self, draw: &Draw, bounds: &Rect, redraw: bool) {
        let cell_w = bounds.w() / GRID_WIDTH as f32;
        let cell_h = bounds.h() / GRID_HEIGHT as f32;
        let start_x = bounds.left() + cell_w / 2.0;
        let start_y = bounds.top() - cell_h / 2.0;

        for row in 0..self.height {
            let row_y = start_y - row as f32 * cell_h;

            for column in 0..self.width {
                let cell = self.get(row, column);

                if redraw || cell.updated {
                    let color = COLORS[cell.value as usize];
                    let cell_x = start_x + column as f32 * cell_w;
                    draw.rect()
                        .x_y(cell_x, row_y)
                        .w_h(cell_w, cell_h)
                        .rgb8(color[0], color[1], color[2]);
                }
            }
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable(false)
        .clear_color(rgb8(BACKGROUND[0], BACKGROUND[1], BACKGROUND[2]))
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .view(view)
        .build()
        .unwrap();

    Model {
        fps: 0.0,
        counter: 0,
        spawn: false,
        spawn_color: 1,
        grid: Grid::new(GRID_WIDTH, GRID_HEIGHT, 0),
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    model.spawn = true;
    model.spawn_color = match button {
        MouseButton::Right => 2,
        MouseButton::Middle => 3,
        _ => 1,
    };
    spawn_cells(app, model, Point2::new(app.mouse.x, app.mouse.y), MOUSE_SPAWN_RADIUS * 2, model.spawn_color);
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.spawn = false;
}

fn mouse_moved(app: &App, model: &mut Model, point: Point2) {
    if model.spawn {
        spawn_cells(app, model, point, MOUSE_SPAWN_RADIUS, model.spawn_color);
    }
}

fn spawn_cells(app: &App, model: &mut Model, point: Point2, radius: i32, fill: u8) {
    let bounds = app.window_rect();
    let pixels_per_row = bounds.h() / GRID_HEIGHT as f32;
    let pixels_per_column = bounds.w() / GRID_WIDTH as f32;
    let grid_row = (bounds.h() / 2.0 - point.y) / pixels_per_row;
    let grid_column = (point.x + bounds.w() / 2.0) / pixels_per_column;

    let grid_row_truncated = clamp(grid_row as usize, 0, GRID_HEIGHT - 1);
    let grid_column_truncated = clamp(grid_column as usize, 0, GRID_WIDTH - 1);

    let brush_row_from = max(grid_row_truncated as i32 - radius, 0) as usize;
    let brush_row_to = min(grid_row_truncated as i32 + radius, GRID_HEIGHT as i32 - 1) as usize;
    let brush_col_from = max(grid_column_truncated as i32 - radius, 0) as usize;
    let brush_col_to = min(grid_column_truncated as i32 + radius, GRID_WIDTH as i32 - 1) as usize;

    for i in brush_row_from..brush_row_to {
        for j in brush_col_from..brush_col_to {
            model.grid.set(i, j, fill);
        }
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.fps = (1000.0).div(update.since_last.as_millis() as f64);

    model.grid.step();

    model.counter += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(rgb8(BACKGROUND[0], BACKGROUND[1], BACKGROUND[2]));
    let draw = app.draw();

    model.grid.display(&draw, &app.window_rect(), true);

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