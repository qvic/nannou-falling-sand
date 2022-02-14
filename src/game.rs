use crate::{
    MaterialId,
    Materials,
    Movement,
    MovementRule,
};
use crate::materials::IndexShift;

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: Option<MaterialId>,
    pub updated: bool,
}

pub struct Simulation {
    grid: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub materials: Materials,
}

#[derive(Eq, PartialEq)]
pub enum CellStatus {
    Empty,
    Occupied(MaterialId),
    Inaccessible,
}

impl Simulation {
    pub fn new(width: usize, height: usize, materials: Materials) -> Self {
        let grid = vec![Cell { value: None, updated: false }; width * height];

        Self { grid, width, height, materials }
    }

    pub fn set(&mut self, row: usize, column: usize, value: Option<MaterialId>) {
        self.grid.get_mut(row * self.width + column)
            .filter(|cell| !cell.updated)
            .map(|cell| {
                cell.value = value;
                cell.updated = true;
            });
    }

    pub fn get(&self, row: usize, column: usize) -> &Cell {
        self.grid.get(row * self.width + column).expect("Tried to get a cell out of bounds")
    }

    pub fn get_status(&self, row: i64, column: i64) -> CellStatus {
        if !self.in_bounds(row, column) { return CellStatus::Inaccessible; }

        self.grid.get((row as usize) * self.width + (column as usize))
            .map(|cell| if cell.updated {
                CellStatus::Inaccessible
            } else if let Some(value) = cell.value {
                CellStatus::Occupied(value)
            } else {
                CellStatus::Empty
            })
            .unwrap()
    }

    pub fn reset(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.updated = false;
        }
    }

    pub fn step(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let cell = self.get(row, column);

                if !cell.updated {
                    if let Some(value) = cell.value {
                        if let Some(movement) = self.apply_rules(row, column, value) {
                            self.move_cell(row, column, value, movement);
                        }
                    }
                }
            }
        }
    }

    fn move_cell(&mut self, row: usize, column: usize, value: MaterialId, movement: Movement) {
        match movement {
            Movement::Stay => {}
            Movement::Move(shift) => {
                self.set(row, column, None);
                self.set((row as i64 + shift.row) as usize, (column as i64 + shift.column) as usize, Some(value));
            }
            Movement::Copy(shift) => {
                self.set((row as i64 + shift.row) as usize, (column as i64 + shift.column) as usize, Some(value));
            }
        }
    }

    fn apply_rules(&self, row: usize, column: usize, value: MaterialId) -> Option<Movement> {
        let rules = self.materials.get_rules(value);

        let row_sgn = row as i64;
        let column_sgn = column as i64;

        rules.iter().find(|rule| self.is_matching_rule(rule, row_sgn, column_sgn, value))
            .map(|rule| rule.movement)
    }

    fn is_matching_rule(&self, rule: &MovementRule, row: i64, column: i64, value: MaterialId) -> bool {
        let empty_satisfied = rule.if_empty.iter().all(|i| self.is_empty(row + i.row, column + i.column));
        let occupied_satisfied = rule.if_occupied.iter().all(|i| self.is_occupied(row + i.row, column + i.column, value));
        let is_valid_movement = self.is_valid_movement(row, column, &rule.movement, value);

        empty_satisfied && occupied_satisfied && is_valid_movement
    }

    fn is_empty(&self, row: i64, column: i64) -> bool {
        self.get_status(row, column) == CellStatus::Empty
    }

    fn is_occupied(&self, row: i64, column: i64, relative_to: MaterialId) -> bool {
        let status = self.get_status(row, column);
        if let CellStatus::Occupied(material) = status {
            relative_to != material
        } else {
            false
        }
    }

    fn is_valid_movement(&self, row: i64, column: i64, movement: &Movement, relative_to: MaterialId) -> bool {
        match movement {
            Movement::Move(shift) => { self.is_valid_shift(row, column, shift, relative_to) }
            Movement::Copy(shift) => { self.is_valid_shift(row, column, shift, relative_to) }
            Movement::Stay => { true }
        }
    }

    fn is_valid_shift(&self, row: i64, column: i64, shift: &IndexShift, relative_to: MaterialId) -> bool {
        let status = self.get_status(row + shift.row, column + shift.column);
        status != CellStatus::Inaccessible && status != CellStatus::Occupied(relative_to)
    }

    fn in_bounds(&self, row: i64, column: i64) -> bool {
        row >= 0 && row < self.height as i64 && column >= 0 && column < self.width as i64
    }
}