use crate::aura;
use std::collections::HashMap;

pub struct Target {
    pub id: i32,
    pub name: String,
    pub dmg: HashMap<i32, u64>,
    pub auras: aura::Auras,
}

impl Target {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            name: format!("Target {}", id),
            dmg: HashMap::new(),
            auras: Default::default(),
        }
    }

    pub fn add_dmg(&mut self, unit_id: i32, dmg: u64) {
        let total = self.dmg.entry(unit_id).or_insert(0);
        *total += dmg;
    }

    pub fn total_dmg(&self) -> u64 {
        self.dmg.values().sum()
    }
}