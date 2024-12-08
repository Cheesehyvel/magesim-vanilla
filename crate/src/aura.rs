use crate::stats::Stats;
use std::collections::HashMap;

pub const ARCANE_POWER: i32 = 12042;
pub const BERSERKING: i32 = 20554;
pub const CLEARCAST: i32 = 12536;
pub const COMBUSTION: i32 = 29977;
pub const EVOCATION: i32 = 12051;
pub const FIRE_VULNERABILITY: i32 = 22959;
pub const INNERVATE: i32 = 29166;
pub const PRESENCE_OF_MIND: i32 = 12043;
pub const POWER_INFUSION: i32 = 10060;
// Trinkets
pub const ESSENCE_OF_SAPPHIRON: i32 = 28779;
pub const MQG: i32 = 23723;
pub const UNSTABLE_POWER: i32 = 24658;
pub const EPHEMERAL_POWER: i32 = 23271;
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
    pub owner_id: i32,
    pub is_shared: bool,
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
            duration,
            stack_increment: 1,
            max_stacks: 1,
            stack_refresh: true,
            is_shared: true,
            ..Default::default()
        }
    }
}


#[derive(Default)]
pub struct Auras {
    auras: Vec<Aura>,
    pub stats: Stats,
}

impl Auras {
    pub fn reset(&mut self) {
        self.auras.clear();
        self.stats.reset();
    }

    pub fn add(&mut self, aura: Aura) -> i32 {
        let stacks = self.stacks(aura.id, aura.owner_id);
        let id = aura.id;
        let owner_id = aura.owner_id;
        let add = aura.stack_increment;

        if stacks == 0 {
            self.auras.push(aura);
        }

        self.add_stacks(id, owner_id, add);

        self.stacks(id, owner_id)
    }

    pub fn get_aura(&self, id: i32, owner_id: i32) -> Option<&Aura> {
        self.auras.iter().find(|aura| aura.id == id && (aura.is_shared || aura.owner_id == owner_id))
    }

    pub fn get_mut_aura(&mut self, id: i32, owner_id: i32) -> Option<&mut Aura> {
        self.auras.iter_mut().find(|aura| aura.id == id && (aura.is_shared || aura.owner_id == owner_id))
    }

    pub fn add_stacks(&mut self, id: i32, owner_id: i32, mut add: i32) {
        if self.has(id, owner_id) {
            let aura = self.get_aura(id, owner_id).unwrap();
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
                    self.auras.retain(|a| a.id != id || !a.is_shared && a.owner_id != owner_id);
                } else {
                    self.get_mut_aura(id, owner_id).unwrap().stacks+= add;
                }
            }
        } 
    }

    pub fn remove(&mut self, id: i32, owner_id: i32) {
        self.auras.retain(|aura| if aura.id == id && (aura.is_shared || aura.owner_id == owner_id) {
            self.stats-= aura.stats * aura.stacks.into();
            false
        } else {
            true
        });
    }

    pub fn stacks(&self, id: i32, owner_id: i32) -> i32 {
        let some_aura = self.get_aura(id, owner_id);
        if some_aura.is_some() {
            return some_aura.unwrap().stacks;
        }

        0
    }

    pub fn has(&self, id: i32, owner_id: i32) -> bool {
        self.get_aura(id, owner_id).is_some()
    }

    pub fn has_any(&self, id: i32) -> bool {
        self.auras.iter().any(|aura| aura.id == id)
    }
}

pub fn mqg() -> Aura {
    Aura::new(MQG, String::from("Mind Quickening Gem"), 20.0)
}

pub fn fire_vulnerability() -> Aura {
    let mut aura = Aura::new(FIRE_VULNERABILITY, String::from("Fire Vulnerability"), 30.0);
    aura.max_stacks = 5;

    aura
}