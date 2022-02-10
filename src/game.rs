#[derive(Copy, Clone)]
pub struct MaterialId(pub u8);

pub struct Material {
    rules: Vec<MovementRule>,
    pub color: u32,
}

impl Material {
    pub fn new(color: u32, rules: Vec<MovementRule>) -> Self {
        Self { color, rules }
    }
}

pub struct MovementRule {
    movement: IndexShift,
    empty: Vec<IndexShift>,
    occupied: Vec<IndexShift>,
}

impl MovementRule {
    pub fn new(movement: IndexShift, empty: Vec<IndexShift>, occupied: Vec<IndexShift>) -> Self {
        Self { movement, empty, occupied }
    }
}

#[derive(Copy, Clone)]
pub struct IndexShift {
    row: i64,
    column: i64,
}

impl IndexShift {
    pub fn new(delta_row: i64, delta_column: i64) -> Self {
        Self { row: delta_row, column: delta_column }
    }
}

#[derive(Copy, Clone)]
pub struct CellValue(pub u8);

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: Option<MaterialId>,
    pub updated: bool,
}

pub struct Simulation {
    grid: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub materials: Vec<Material>,
}

impl Simulation {
    pub fn new(width: usize, height: usize, materials: Vec<Material>) -> Self {
        let grid = vec![Cell { value: None, updated: false }; width * height];

        Self { grid, width, height, materials }
    }

    pub fn set(&mut self, row: usize, column: usize, value: Option<MaterialId>) {
        let cell = &mut self.grid[row * self.width + column];
        cell.value = value;
        cell.updated = true;
    }

    pub fn get(&self, row: usize, column: usize) -> &Cell {
        &self.grid[row * self.width + column]
    }

    pub fn prepare(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.updated = false;
        }
    }

    pub fn step(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let current_cell = self.get(row, column);

                if !current_cell.updated && current_cell.value.is_some() {
                    self.process_cell(row, column);
                }
            }
        }
    }

    fn process_cell(&mut self, row: usize, column: usize) {
        // TODO pass cell via argument
        let cell = self.get(row, column);
        let cell_color = cell.value;
        let material = &self.materials[cell.value.unwrap().0 as usize];
        let row_sgn = row as i64;
        let column_sgn = column as i64;

        let mut matched_rule = None;
        // TODO iterator find()
        for rule in &material.rules {
            let mut failed = false;
            for ind in &rule.empty {
                if !self.is_free(row_sgn + ind.row, column_sgn + ind.column) {
                    failed = true;
                    break;
                }
            }
            if failed { continue; }
            for ind in &rule.occupied {
                if self.is_free(row_sgn + ind.row, column_sgn + ind.column) {
                    failed = true;
                    break;
                }
            }
            if failed { continue; }
            matched_rule = Some(rule);
            break;
        }
        if matched_rule.is_none() { return; }

        let movement = matched_rule.unwrap().movement;

        self.set(row, column, None);
        // TODO check bounds
        self.set((row_sgn + movement.row) as usize, (column_sgn + movement.column) as usize, cell_color);
    }

    fn is_free(&self, row: i64, column: i64) -> bool {
        if row < 0 || row > self.height as i64 - 1 || column < 0 || column > self.width as i64 - 1 {
            false
        } else {
            self.get(row as usize, column as usize).value.is_none()
        }
    }
}