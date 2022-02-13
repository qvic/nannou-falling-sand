use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(try_from = "ColorHexString")]
#[serde(into = "ColorHexString")]
pub struct MaterialColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl MaterialColor {
    pub fn new(hex: u32) -> Self {
        let b: u8 = (hex & 0xFF) as u8;
        let g: u8 = ((hex >> 8) & 0xFF) as u8;
        let r: u8 = ((hex >> 16) & 0xFF) as u8;
        Self {r, g, b}
    }
}

#[derive(Serialize, Deserialize)]
struct ColorHexString(String);

impl TryFrom<ColorHexString> for MaterialColor {
    type Error = std::num::ParseIntError;

    fn try_from(other: ColorHexString) -> Result<Self, Self::Error> {
        u32::from_str_radix(other.0.as_str(), 16)
            .map(|hex| MaterialColor::new(hex))
    }
}

impl From<MaterialColor> for ColorHexString {
    fn from(other: MaterialColor) -> Self {
        ColorHexString(format!("{:X}{:X}{:X}", other.r, other.g, other.b))
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterialId(pub u8);

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub rules: Vec<MovementRule>,
    pub color: MaterialColor,
    pub key: String,
}

impl Material {
    pub fn new(id: u8, name: &str, key: &str, color: MaterialColor, rules: Vec<MovementRule>) -> Self {
        Self { id: MaterialId(id), name: String::from(name), key: String::from(key), color, rules }
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Movement {
    Stay,
    Move(IndexShift),
    Copy(IndexShift)
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct IndexShift {
    pub row: i64,
    pub column: i64,
}

impl IndexShift {
    pub fn new(delta_row: i64, delta_column: i64) -> Self {
        Self { row: delta_row, column: delta_column }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Materials {
    map: HashMap<MaterialId, Material>,
    pub background: MaterialColor,
}

impl Materials {
    pub fn new(background: MaterialColor, list: Vec<Material>) -> Self {
        assert_ne!(list.len(), 0, "At least one material should be added");
        Self { background, map: list.into_iter().map(|m| (m.id, m)).collect() }
    }

    pub fn get_color(&self, cell_value: Option<MaterialId>) -> MaterialColor {
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