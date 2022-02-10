mod game;

use std::cmp::min;
use std::ops::Add;
use nannou::color::rgb_u32;

use nannou::prelude::*;

use game::Material;
use crate::game::{IndexShift, MaterialId, MovementRule, Simulation};

const WINDOW_TITLE: &'static str = "Falling sand";

const WINDOW_WIDTH_PX: u32 = 800;
const WINDOW_HEIGHT_PX: u32 = 800;

const GRID_WIDTH_CELLS: usize = 100;
const GRID_HEIGHT_CELLS: usize = 100;

const MOUSE_SPAWN_RADIUS: u8 = 1;

const BACKGROUND: u32 = 0xEEE6CE;

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
        .clear_color(rgb_u32(BACKGROUND))
        .view(view)
        .event(event)
        .build()
        .unwrap();

    let materials = vec![
        Material::new(0x2EB086, vec![
            MovementRule::new(IndexShift::new(1, 0), vec![IndexShift::new(1, 0)], vec![]),
            MovementRule::new(IndexShift::new(1, -1), vec![IndexShift::new(1, -1)], vec![]),
            MovementRule::new(IndexShift::new(1, 1), vec![IndexShift::new(1, 1)], vec![]),
        ]),
        Material::new(0xB8405E, vec![]),
    ];

    Model {
        fps: 0.0,
        brush: Brush::new(),
        game: GameView::new(GRID_WIDTH_CELLS, GRID_HEIGHT_CELLS, app.window_rect(), BACKGROUND, materials),
    }
}


fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(button) => {
            model.brush.active = true;
            model.brush.fill = match button {
                MouseButton::Right => Some(MaterialId(1)),
                MouseButton::Middle => None,
                _ => Some(MaterialId(0)),
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

    model.game.prepare();

    if model.brush.active {
        model.game.spawn(app.mouse.position(), model.brush.radius, model.brush.fill);
    }

    model.game.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.game.display(&draw);

    if DRAW_FPS && frame.nth() % REDRAW_FPS_FRAMES == 0 {
        draw_fps(app, &draw, model);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    fps: f64,
    brush: Brush,
    game: GameView,
}

struct Brush {
    active: bool,
    radius: u8,
    fill: Option<MaterialId>,
}

impl Brush {
    pub fn new() -> Self {
        Self { active: false, radius: MOUSE_SPAWN_RADIUS, fill: None }
    }
}

struct GameView {
    simulation: Simulation,
    bounds: Rect,
    colors: Vec<Rgb8>,
}

impl GameView {
    fn new(width: usize, height: usize, bounds: Rect, background_color: u32, materials: Vec<Material>) -> Self {
        let mut colors = Vec::with_capacity(materials.len() + 1);
        colors.push(rgb_u32(background_color));
        for material in materials.iter() {
            colors.push(rgb_u32(material.color));
        }

        Self { simulation: Simulation::new(width, height, materials), bounds, colors }
    }

    fn prepare(&mut self) {
        self.simulation.prepare();
    }

    fn step(&mut self) {
        self.simulation.step();
    }

    fn display(&self, draw: &Draw) {
        let cell_width = self.bounds.w() / self.simulation.width as f32;
        let cell_height = self.bounds.h() / self.simulation.height as f32;

        let start_x = self.bounds.left() + cell_width / 2.0;
        let start_y = self.bounds.top() - cell_height / 2.0;

        for row in 0..self.simulation.height {
            let row_y = start_y - row as f32 * cell_height;

            for column in 0..self.simulation.width {
                let cell = self.simulation.get(row, column);

                if cell.updated {
                    let color_index = cell.value.map_or(0, |v| v.0 + 1) as usize;
                    let color = self.colors[color_index];
                    let cell_x = start_x + column as f32 * cell_width;
                    Self::draw_cell(draw, cell_x, row_y, cell_width, cell_width, color);
                }
            }
        }
    }

    fn draw_cell(draw: &Draw, x: f32, y: f32, w: f32, h: f32, color: Rgb8) {
        draw.rect()
            .x_y(x, y)
            .w_h(w, h)
            .color(color);
    }

    fn spawn(&mut self, mouse: Vec2, radius: u8, material: Option<MaterialId>) {
        let r = radius as usize;
        let cell_width = self.bounds.w() / self.simulation.width as f32;
        let cell_height = self.bounds.h() / self.simulation.height as f32;

        let row = ((self.bounds.y() + self.bounds.h() / 2.0 - mouse.y) / cell_height) as usize;
        let column = ((self.bounds.x() + self.bounds.w() / 2.0 + mouse.x) / cell_width) as usize;

        let brush_row_from = row.saturating_sub(r);
        let brush_row_to = min(row + r, self.simulation.height - 1);

        let brush_col_from = column.saturating_sub(r);
        let brush_col_to = min(column + r, self.simulation.width - 1);

        for i in brush_row_from..=brush_row_to {
            for j in brush_col_from..=brush_col_to {
                if self.simulation.get(i, j).value.is_none() || material.is_none() {
                    self.simulation.set(i, j, material);
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
