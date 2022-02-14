use std::cmp::min;
use std::ops::Div;

use nannou::prelude::*;

use crate::{MaterialColor, MaterialId, Materials, Simulation};

impl Into<Rgb8> for MaterialColor {
    fn into(self) -> Rgb8 {
        Rgb8::new(self.r, self.g, self.b)
    }
}

pub struct GameView {
    simulation: Simulation,
    bounds: Rect,
    cell_size: Vec2,
}

impl GameView {
    pub fn new(width: usize, height: usize, bounds: Rect, materials: Materials) -> Self {
        let cell_size = bounds.wh().div(vec2(width as f32, height as f32));
        Self { simulation: Simulation::new(width, height, materials), bounds, cell_size }
    }

    pub fn materials(&self) -> &Materials {
        &self.simulation.materials
    }

    pub fn prepare(&mut self) {
        self.simulation.reset();
    }

    pub fn step(&mut self) {
        self.simulation.step();
    }

    pub fn display(&self, draw: &Draw) {
        let cell_width = self.cell_size.x;
        let cell_height = self.cell_size.y;

        let start_x = self.bounds.left() + cell_width / 2.0;
        let start_y = self.bounds.top() - cell_height / 2.0;

        for row in 0..self.simulation.height {
            let row_y = start_y - row as f32 * cell_height;

            for column in 0..self.simulation.width {
                let cell = self.simulation.get(row, column);

                if cell.updated {
                    let color = self.simulation.materials.get_color(cell.value);
                    let cell_x = start_x + column as f32 * cell_width;
                    Self::draw_cell(draw, cell_x, row_y, cell_width, cell_width, color);
                }
            }
        }
    }

    fn draw_cell(draw: &Draw, x: f32, y: f32, w: f32, h: f32, color: MaterialColor) {
        draw.rect()
            .x_y(x, y)
            .w_h(w, h)
            .color(Into::<Rgb8>::into(color));
    }

    pub fn spawn(&mut self, mouse: Vec2, radius: u8, material: Option<MaterialId>) {
        let r = radius as usize;
        let cell_width = self.cell_size.x;
        let cell_height = self.cell_size.y;

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

    pub fn clear(&mut self) {
        for i in 0..self.simulation.height {
            for j in 0..self.simulation.width {
                self.simulation.set(i, j, None);
            }
        }
    }
}