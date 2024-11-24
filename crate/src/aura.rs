use crate::stats::Stats;
use std::collections::HashMap;

pub const ARCANE_POWER: i32 = 12042;
pub const BERSERKING: i32 = 20554;
pub const CLEARCAST: i32 = 12536;
pub const COMBUSTION: i32 = 29977;
pub const EVOCATION: i32 = 12051;
pub const PRESENCE_OF_MIND: i32 = 12043;
pub const INNERVATE: i32 = 29166;
pub const POWER_INFUSION: i32 = 10060;
// Trinkets
pub const ESSENCE_OF_SAPPHIRON: i32 = 28779;
pub const MQG: i32 = 23723;
pub const UNSTABLE_POWER: i32 = 24658;
pub const EPHEMERAL_POWER: i32 = 23271;
pub const ARCANE_POTENCY: i32 = 24544;
pub const OBSIDIAN_INSIGHT: i32 = 26166;
pub const BLUE_DRAGON: i32 = 23688;
pub const NAT_PAGLE: i32 = 24610;
pub const CHROMATIC_INFUSION: i32 = 27675;
pub const BURST_OF_KNOWLEDGE: i32 = 15646;
pub const CHAOS_FIRE: i32 = 24389;
// Items / sets
pub const NETHERWIND_FOCUS: i32 = 22007;
pub const ENIGMAS_ANSWER: i32 = 26129;


#[derive(Default, Clone)]
pub struct Aura {
    pub id: i32,
    pub name: String,
    pub duration: f64,
    pub stacks: i32,
    pub stack_increment: i32,
    pub max_stacks: i32,
    pub stack_refresh: bool,
    pub show_refresh: bool,
    pub should_snapshot: bool,
    pub is_snapshot: bool,
    pub is_hidden: bool,
    pub t_gained: f64,
    pub t_refreshed: f64,
    pub t_expires: f64,
    pub stats: Stats,
}

impl Aura {
    pub fn new(id: i32, name: String, duration: f64) -> Self {
        Self {
            id,
            name,
            stacks: 1,
            stack_increment: 1,
            max_stacks: 1,
            stack_refresh: true,
            ..Default::default()
        }
    }
}


#[derive(Default)]
pub struct Auras {
    auras: HashMap<i32, Aura>,
    pub stats: Stats,
}

impl Auras {
    pub fn reset(&mut self) {
        self.auras.clear();
        self.stats.reset();
    }

    pub fn add(&mut self, aura: Aura) -> i32 {
        let stacks = self.stacks(aura.id);
        let id = aura.id;
        let add = aura.stack_increment;

        if stacks == 0 {
            self.auras.insert(id, aura);
        }

        self.add_stacks(id, add);

        self.stacks(id)
    }

    pub fn add_stacks(&mut self, id: i32, stacks: i32) {
        if self.auras.contains_key(&id) {
            let aura = self.auras.get_mut(&id).unwrap();
            let mut add = stacks;
            let stacks = aura.stacks;

            if add < 0 && stacks == 0 {
                add = aura.max_stacks;
            } else if stacks + add > aura.max_stacks {
                add = aura.max_stacks - stacks;
            } else if stacks + add < 0 {
                add = -stacks;
            }

            if add != 0 {
                self.stats+= aura.stats * add.into();

                if stacks + add == 0 {
                    self.auras.remove(&id);
                } else {
                    aura.stacks+= add;
                }
            }
        }
    }

    pub fn remove(&mut self, id: i32) {
        if self.auras.contains_key(&id) {
            let aura = &self.auras[&id];
            self.stats-= aura.stats * aura.stacks.into();
            self.auras.remove(&id);
        }
    }

    pub fn stacks(&self, id: i32) -> i32 {
        if self.auras.contains_key(&id) {
            return self.auras[&id].stacks;
        }

        0
    }

    pub fn has(&self, id: i32) -> bool {
        self.auras.contains_key(&id)
    }

}

pub fn mqg() -> Aura {
    Aura::new(MQG, String::from("Mind Quickening Gem"), 20.0)
}