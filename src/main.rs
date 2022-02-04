use std::cmp::{min};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

use nannou::prelude::*;
use nannou::rand;
use crate::rand::Rng;

use crate::rand::rngs::ThreadRng;

const WINDOW_TITLE: &'static str = "Falling sand";

const WINDOW_WIDTH_PX: u32 = 800;
const WINDOW_HEIGHT_PX: u32 = 800;

const GRID_WIDTH_CELLS: usize = 100;
const GRID_HEIGHT_CELLS: usize = 100;

const CELL_WIDTH_PX: f32 = WINDOW_WIDTH_PX as f32 / GRID_WIDTH_CELLS as f32;
const CELL_HEIGHT_PX: f32 = WINDOW_HEIGHT_PX as f32 / GRID_HEIGHT_CELLS as f32;

const MOUSE_SPAWN_RADIUS: u8 = 1;

const BACKGROUND: Rgb8 = rgb8(255, 237, 219);
const COLORS: [Rgb8; 4] = [
    BACKGROUND,
    rgb8(184, 64, 94),
    rgb8(46, 176, 134),
    rgb8(49, 53, 82),
];

const DRAW_FPS: bool = true;
const REDRAW_FPS_FRAMES: u64 = 8;

struct RowColumnIndex(usize, usize);

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(WINDOW_TITLE)
        .size(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX)
        .resizable(false)
        .clear_color(BACKGROUND)
        .view(view)
        .event(event)
        .build()
        .unwrap();

    Model {
        fps: 0.0,
        counter: 0,
        brush: Brush { fill: 1, active: false, radius: MOUSE_SPAWN_RADIUS },
        grid: Grid::new(GRID_WIDTH_CELLS, GRID_HEIGHT_CELLS, 0),
    }
}


fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(button) => {
            model.brush.active = true;
            model.brush.fill = match button {
                MouseButton::Right => 2,
                MouseButton::Middle => 0,
                _ => 1,
            };
        }
        MouseReleased(_) => {
            model.brush.active = false;
        }
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.fps = 1000.0 / update.since_last.as_millis() as f64;

    model.grid.prepare();

    if model.brush.active {
        let grid_coords = convert_global_to_grid_coords(app.mouse.position());
        model.grid.spawn(grid_coords.0, grid_coords.1, model.brush.radius, model.brush.fill);
    }

    model.grid.step();

    model.counter += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.grid.display(&draw);

    if DRAW_FPS && frame.nth() % REDRAW_FPS_FRAMES == 0 {
        draw_fps(app, &draw, model);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    fps: f64,
    counter: u64,
    brush: Brush,
    grid: Grid,
}

type CellValue = u8;

struct Brush {
    active: bool,
    radius: u8,
    fill: CellValue,
}

#[derive(Copy, Clone)]
struct Cell {
    value: CellValue,
    updated: bool,
}

struct Grid {
    buffer: Vec<Cell>,
    width: usize,
    height: usize,
    rng: ThreadRng,
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
    fn new(width: usize, height: usize, fill: CellValue) -> Self {
        let buffer = vec![Cell { value: fill, updated: false }; width * height];
        let rng = rand::thread_rng();

        Grid { buffer, width, height, rng }
    }

    fn set(&mut self, row: usize, column: usize, value: CellValue) {
        let cell = &mut self.buffer[row * self.width + column];
        cell.value = value;
        cell.updated = true;
    }

    fn get(&self, row: usize, column: usize) -> &Cell {
        &self.buffer[row * self.width + column]
    }

    fn prepare(&mut self) {
        for cell in self.buffer.iter_mut() {
            cell.updated = false;
        }
    }

    fn step(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let current_cell = self.get(row, column);
                let current_value = current_cell.value;

                if !current_cell.updated && current_value > 0 {
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

    fn display(&self, draw: &Draw) {
        let start_x = - (WINDOW_WIDTH_PX as f32) / 2.0 + CELL_WIDTH_PX / 2.0;
        let start_y = WINDOW_HEIGHT_PX as f32 / 2.0 - CELL_HEIGHT_PX / 2.0;

        for row in 0..self.height {
            let row_y = start_y - row as f32 * CELL_HEIGHT_PX;

            for column in 0..self.width {
                let cell = self.get(row, column);

                if cell.updated {
                    let cell_x = start_x + column as f32 * CELL_WIDTH_PX;
                    Grid::draw_cell(draw, cell_x, row_y, CELL_WIDTH_PX, CELL_HEIGHT_PX, cell.value);
                }
            }
        }
    }

    fn draw_cell(draw: &Draw, x: f32, y: f32, w: f32, h: f32, value: u8) {
        let color = COLORS[value as usize];
        draw.rect()
            .x_y(x, y)
            .w_h(w, h)
            .color(color);
    }

    fn spawn(&mut self, row: usize, column: usize, radius: u8, fill: CellValue) {
        let r = radius as usize;

        let brush_row_from = row.saturating_sub(r);
        let brush_row_to = min(row + r, GRID_HEIGHT_CELLS - 1);

        let brush_col_from = column.saturating_sub(r);
        let brush_col_to = min(column + r, GRID_WIDTH_CELLS - 1);

        for i in brush_row_from..=brush_row_to {
            for j in brush_col_from..=brush_col_to {
                if self.get(i, j).value == 0 || fill == 0 {
                    self.set(i, j, fill);
                }
            }
        }
    }
}

fn convert_global_to_grid_coords(vec: Vec2) -> RowColumnIndex {
    let row = (WINDOW_HEIGHT_PX as f32 / 2.0 - vec.y) / CELL_HEIGHT_PX;
    let column = (vec.x + WINDOW_WIDTH_PX as f32 / 2.0) / CELL_WIDTH_PX;

    RowColumnIndex(clamp(row as usize, 0, GRID_HEIGHT_CELLS - 1),
                   clamp(column as usize, 0, GRID_WIDTH_CELLS - 1))
}

fn draw_fps(app: &App, draw: &Draw, model: &Model) {
    let text_xy = app.main_window().rect().top_left().add(vec2(50.0, -15.0));
    let text_wh = vec2(100.0, 30.0);
    let font_size = 20;
    let text = format!("{:.2} FPS", model.fps);

    draw.rect()
        .color(BLACK)
        .xy(text_xy)
        .wh(text_wh);

    draw.text(&text)
        .color(WHITE)
        .xy(text_xy)
        .wh(text_wh)
        .font_size(font_size);
}

const fn rgb8(red: u8, green: u8, blue: u8) -> Rgb8 {
    Rgb8 { red, green, blue, standard: PhantomData }
}