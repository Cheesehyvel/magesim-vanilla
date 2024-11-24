use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Cooldown {
    pub id: i32,
    pub name: String,
    pub duration: f64,
    pub t_gained: f64,
    pub t_expires: f64,
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
    cooldowns: HashMap<i32, Cooldown>,
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

