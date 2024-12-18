use std::collections::HashMap;

pub const TRINKET_POWER: i32 = 1;

#[derive(Default, Clone)]
pub struct Cooldown {
    pub id: i32,
    pub name: String,
    pub duration: f64,
    pub t_gained: f64,
    pub t_expires: f64,
    pub is_hidden: bool,
}

impl Cooldown {
    pub fn new(id: i32, name: String, duration: f64) -> Self {
        Self {
            id,
            name,
            duration,
            ..Default::default()
        }
    }
}


#[derive(Default)]
pub struct Cooldowns {
    pub cooldowns: HashMap<i32, Cooldown>,
}

impl Cooldowns {
    pub fn reset(&mut self) {
        self.cooldowns.clear();
    }

    pub fn add(&mut self, cooldown: Cooldown) {
        self.remove(cooldown.id);
        self.cooldowns.insert(cooldown.id, cooldown);
    }

    pub fn remove(&mut self, id: i32) {
        if self.cooldowns.contains_key(&id) {
            self.cooldowns.remove(&id);
        }
    }

    pub fn duration(&self, id: i32) -> f64 {
        if self.cooldowns.contains_key(&id) {
            return self.cooldowns.get(&id).unwrap().duration;
        }

        0.0
    }

    pub fn has(&self, id: i32) -> bool {
        self.cooldowns.contains_key(&id)
    }

}


/*
 * Defined cooldowns
 */

pub fn shared_trinket_power(duration: f64) -> Cooldown {
    let mut cooldown = Cooldown::new(TRINKET_POWER, String::from("Trinket Power"), duration);
    cooldown.is_hidden = true;

    cooldown
}
