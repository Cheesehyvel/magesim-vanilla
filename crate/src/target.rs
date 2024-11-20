use crate::aura;

pub struct Target {
    pub id: i32,
    pub name: String,
    pub dmg: u64,
    pub auras: aura::Auras,
}

impl Target {
    pub fn new(id: i32) -> Self {
        Self {
            id: id,
            name: format!("Target {}", id),
            dmg: 0,
            auras: Default::default(),
        }
    }
}