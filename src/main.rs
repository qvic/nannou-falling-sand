use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Add;

use nannou::prelude::*;
use nannou::rand;

use crate::rand::Rng;
use crate::rand::rngs::ThreadRng;

const WINDOW_TITLE: &'static str = "Falling sand";

const WINDOW_WIDTH_PX: u32 = 800;
const WINDOW_HEIGHT_PX: u32 = 800;

const GRID_WIDTH_CELLS: usize = 100;
const GRID_HEIGHT_CELLS: usize = 100;

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
        simulation: Simulation {
            grid: Grid::new(GRID_WIDTH_CELLS, GRID_HEIGHT_CELLS, 0),
            bounds: app.window_rect(),
        },
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

    model.simulation.prepare();

    if model.brush.active {
        model.simulation.spawn(app.mouse.position(), model.brush.radius, model.brush.fill);
    }

    model.simulation.step();

    model.counter += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.simulation.display(&draw);

    if DRAW_FPS && frame.nth() % REDRAW_FPS_FRAMES == 0 {
        draw_fps(app, &draw, model);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    fps: f64,
    counter: u64,
    brush: Brush,
    simulation: Simulation,
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

struct Simulation {
    grid: Grid,
    bounds: Rect,
}

impl Simulation {

    fn prepare(&mut self) {
        self.grid.prepare();
    }

    fn step(&mut self) {
        self.grid.step();
    }

    fn display(&self, draw: &Draw) {
        let cell_width = self.bounds.w() / self.grid.width as f32;
        let cell_height = self.bounds.h() / self.grid.height as f32;

        let start_x = self.bounds.left() + cell_width / 2.0;
        let start_y = self.bounds.top() - cell_height / 2.0;

        for row in 0..self.grid.height {
            let row_y = start_y - row as f32 * cell_height;

            for column in 0..self.grid.width {
                let cell = self.grid.get(row, column);

                if cell.updated {
                    let cell_x = start_x + column as f32 * cell_width;
                    Simulation::draw_cell(draw, cell_x, row_y, cell_width, cell_width, cell.value);
                }
            }
        }
    }

    fn draw_cell(draw: &Draw, x: f32, y: f32, w: f32, h: f32, value: CellValue) {
        let color = COLORS[value as usize];
        draw.rect()
            .x_y(x, y)
            .w_h(w, h)
            .color(color);
    }

    fn spawn(&mut self, mouse: Vec2, radius: u8, fill: CellValue) {
        let r = radius as usize;
        let cell_width = self.bounds.w() / self.grid.width as f32;
        let cell_height = self.bounds.h() / self.grid.height as f32;

        let row = ((self.bounds.y() + self.bounds.h() / 2.0 - mouse.y) / cell_height) as usize;
        let column = ((self.bounds.x() + self.bounds.w() / 2.0 + mouse.x) / cell_width) as usize;

        let brush_row_from = row.saturating_sub(r);
        let brush_row_to = min(row + r, self.grid.height - 1);

        let brush_col_from = column.saturating_sub(r);
        let brush_col_to = min(column + r, self.grid.width - 1);

        for i in brush_row_from..=brush_row_to {
            for j in brush_col_from..=brush_col_to {
                if self.grid.get(i, j).value == 0 || fill == 0 {
                    self.grid.set(i, j, fill);
                }
            }
        }
    }
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

    fn get_bottom_row_availability(&self, row: usize, column: usize) -> [bool; 3] {
        let last_row = row == self.height - 1;
        let bottom = !last_row && self.get(row + 1, column).value == 0;
        let bottom_left = !last_row && column > 0 && self.get(row + 1, column - 1).value == 0;
        let bottom_right = !last_row && column < self.width - 1 && self.get(row + 1, column + 1).value == 0;

        [bottom_left, bottom, bottom_right]
    }

    fn step(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let current_cell = self.get(row, column);
                let current_value = current_cell.value;

                if !current_cell.updated && current_value > 0 {
                    match self.get_bottom_row_availability(row, column) {
                        [_, true, _] => {
                            self.set(row, column, 0);
                            self.set(row + 1, column, current_value);
                        }
                        [true, _, true] => {
                            let go_left: bool = self.rng.gen();
                            let dx = (go_left as i32) * 2 - 1; // map 0 1 to -1 1
                            let new_column = (column as i32 + dx) as usize;
                            self.set(row, column, 0);
                            self.set(row + 1, new_column, current_value);
                        }
                        [true, _, _] => {
                            self.set(row, column, 0);
                            self.set(row + 1, column - 1, current_value);
                        }
                        [_, _, true] => {
                            self.set(row, column, 0);
                            self.set(row + 1, column + 1, current_value);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
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