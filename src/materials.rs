use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct MaterialId(pub u8);

pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub rules: Vec<MovementRule>,
    pub color: u32,
    pub key: String,
}

impl Material {
    pub fn new(id: u8, name: &str, key: &str, color: u32, rules: Vec<MovementRule>) -> Self {
        Self { id: MaterialId(id), name: String::from(name), key: String::from(key), color, rules }
    }
}

pub struct MovementRule {
    pub movement: Movement,
    pub empty: Vec<IndexShift>,
    pub occupied: Vec<IndexShift>,
}

impl MovementRule {
    pub fn new(movement: Movement, empty: Vec<IndexShift>, occupied: Vec<IndexShift>) -> Self {
        Self { movement, empty, occupied }
    }
}

#[derive(Copy, Clone)]
pub enum Movement {
    Stay,
    Move(IndexShift),
    Copy(IndexShift)
}

#[derive(Copy, Clone)]
pub struct IndexShift {
    pub row: i64,
    pub column: i64,
}

impl IndexShift {
    pub fn new(delta_row: i64, delta_column: i64) -> Self {
        Self { row: delta_row, column: delta_column }
    }
}

pub struct Materials {
    map: HashMap<MaterialId, Material>,
    pub background: u32,
}

impl Materials {
    pub fn new(background: u32, list: Vec<Material>) -> Self {
        assert_ne!(list.len(), 0, "At least one material should be added");
        Self { background, map: list.into_iter().map(|m| (m.id, m)).collect() }
    }

    pub fn get_color(&self, cell_value: Option<MaterialId>) -> u32 {
        match cell_value {
            Some(id) => self.get_by_id(id).color,
            None => self.background
        }
    }

    pub fn get_name(&self, cell_value: Option<MaterialId>) -> &str {
        match cell_value {
            Some(id) => self.get_by_id(id).name.as_str(),
            None => "Eraser"
        }
    }

    pub fn get_rules(&self, id: MaterialId) -> &Vec<MovementRule> {
        &self.get_by_id(id).rules
    }

    fn get_by_id(&self, id: MaterialId) -> &Material {
        self.map.get(&id).expect("Could not find material for given id")
    }

    pub fn get_id_by_key(&self, key: &str) -> Option<MaterialId> {
        for x in self.map.iter() {
            if x.1.key == key {
                return Some(x.0.clone());
            }
        }
        None
    }
}