use crate::aura;
use std::collections::HashMap;

pub struct Target {
    pub id: i32,
    pub name: String,
    pub dmg: HashMap<i32, u64>,
    pub auras: aura::Auras,
    pub ignite_dmg: f64,
    pub ignite_modifier: f64,
    pub ignite_t: f64,
    pub ignite_stacks: u8,
    pub ignite_owner_id: i32,
}

impl Target {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            name: format!("Target {}", id),
            dmg: HashMap::new(),
            auras: Default::default(),
            ignite_dmg: 0.0,
            ignite_modifier: 1.0,
            ignite_t: 0.0,
            ignite_stacks: 0,
            ignite_owner_id: 0,
        }
    }

    pub fn add_dmg(&mut self, unit_id: i32, dmg: u64) {
        let total = self.dmg.entry(unit_id).or_insert(0);
        *total += dmg;
    }

    pub fn total_dmg(&self) -> u64 {
        self.dmg.values().sum()
    }

    pub fn reset_ignite(&mut self) {
        self.ignite_dmg = 0.0;
        self.ignite_modifier = 1.0;
        self.ignite_t = 0.0;
        self.ignite_stacks = 0;
        self.ignite_owner_id = 0;
    }
}