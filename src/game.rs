use crate::{
    MovementRule,
    Movement,
    Materials,
    MaterialId
};

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

impl Simulation {
    pub fn new(width: usize, height: usize, materials: Materials) -> Self {
        let grid = vec![Cell { value: None, updated: false }; width * height];

        Self { grid, width, height, materials }
    }

    pub fn set(&mut self, row: usize, column: usize, value: Option<MaterialId>) {
        if !self.in_bounds(row as i64, column as i64) { panic!("Tried to set a cell out of bounds"); }

        let cell = &mut self.grid[row * self.width + column];
        cell.value = value;
        cell.updated = true;
    }

    pub fn get(&self, row: usize, column: usize) -> &Cell {
        if !self.in_bounds(row as i64, column as i64) { panic!("Tried to get a cell out of bounds"); }

        &self.grid[row * self.width + column]
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
        let empty_satisfied = rule.empty.iter().all(|i| self.is_empty(row + i.row, column + i.column));
        let occupied_satisfied = rule.occupied.iter().all(|i| self.is_occupied(row + i.row, column + i.column, value));

        empty_satisfied && occupied_satisfied
    }

    fn is_empty(&self, row: i64, column: i64) -> bool {
        if self.in_bounds(row, column) {
            let cell = self.get(row as usize, column as usize);
            !cell.updated && cell.value.is_none()
        } else {
            false
        }
    }

    fn is_occupied(&self, row: i64, column: i64, current_value: MaterialId) -> bool {
        if self.in_bounds(row, column) {
            let cell = self.get(row as usize, column as usize);
            !cell.updated && cell.value.filter(|&v| v != current_value).is_some()
        } else {
            false
        }
    }

    fn in_bounds(&self, row: i64, column: i64) -> bool {
        row >= 0 && row < self.height as i64 && column >= 0 && column < self.width as i64
    }
}