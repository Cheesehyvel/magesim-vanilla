use crate::stats::Stats;
use std::collections::HashMap;

pub const ARCANE_POWER: i32 = 12042;
pub const BERSERKING: i32 = 20554;
pub const CLEARCAST: i32 = 12536;
pub const COMBUSTION: i32 = 29977;
pub const EVOCATION: i32 = 12051;
pub const FIRE_VULNERABILITY: i32 = 22959;
pub const INNERVATE: i32 = 29166;
pub const MANA_TIDE: i32 = 17359;
pub const PRESENCE_OF_MIND: i32 = 12043;
pub const POWER_INFUSION: i32 = 10060;
pub const WINTERS_CHILL: i32 = 12579;
// Trinkets
pub const ARCANE_POTENCY: i32 = 24544;
pub const BLUE_DRAGON: i32 = 23688;
pub const BURST_OF_KNOWLEDGE: i32 = 15646;
pub const CHAOS_FIRE: i32 = 24389;
pub const CHROMATIC_INFUSION: i32 = 27675;
pub const EPHEMERAL_POWER: i32 = 23271;
pub const ESSENCE_OF_SAPPHIRON: i32 = 28779;
pub const MIND_QUICKENING: i32 = 23723;
pub const NAT_PAGLE: i32 = 24610;
pub const OBSIDIAN_INSIGHT: i32 = 26166;
pub const UNSTABLE_POWER: i32 = 24658;
// Items / sets
pub const ENIGMAS_ANSWER: i32 = 26129;
pub const NETHERWIND_FOCUS: i32 = 22007;


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
    pub owner_id: i32,
    pub is_shared: bool,
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
        if let Some(aura) = some_aura {
            return aura.stacks;
        }
        0
    }

    pub fn has(&self, id: i32, owner_id: i32) -> bool {
        self.get_aura(id, owner_id).is_some()
    }

    pub fn has_any(&self, id: i32) -> bool {
        self.auras.iter().any(|aura| aura.id == id)
    }

    pub fn can_react(&self, id: i32, owner_id: i32, t: f64) -> bool {
        self.auras.iter().any(|aura| aura.id == id && (aura.is_shared || aura.owner_id == owner_id) && aura.t_gained <= t)
    }

    pub fn can_react_any(&self, id: i32, t: f64) -> bool {
        self.auras.iter().any(|aura| aura.id == id && aura.t_gained <= t)
    }
}


/*
 * Defined auras
 */

pub fn arcane_potency() -> Aura {
    Aura::new(ARCANE_POTENCY, String::from("Arcane Potency"), 20.0)
}

pub fn arcane_power() -> Aura {
    Aura::new(ARCANE_POWER, String::from("Arcane Power"), 15.0)
}

pub fn berserking() -> Aura {
    Aura::new(BERSERKING, String::from("Berserking"), 10.0)
}

pub fn blue_dragon() -> Aura {
    Aura::new(BLUE_DRAGON, String::from("Blue Dragon"), 15.0)
}

pub fn burst_of_knowledge() -> Aura {
    Aura::new(BURST_OF_KNOWLEDGE, String::from("Burst of Knowledge"), 10.0)
}

pub fn chaos_fire() -> Aura {
    Aura::new(CHAOS_FIRE, String::from("Chaos Fire"), 8.0)
}

pub fn chromatic_infusion() -> Aura {
    let mut aura = Aura::new(CHROMATIC_INFUSION, String::from("Chromatic Infusion"), 15.0);
    aura.stats.sp = 100.0;
    aura
}

pub fn clearcast() -> Aura {
    Aura::new(CLEARCAST, String::from("Clearcast"), 15.0)
}

pub fn combustion() -> Aura {
    let mut aura = Aura::new(COMBUSTION, String::from("Combustion"), 1000.0);
    aura.max_stacks = 20;
    aura
}

pub fn enigmas_answer() -> Aura {
    let mut aura = Aura::new(ENIGMAS_ANSWER, String::from("Enigma's Answer"), 20.0);
    aura.max_stacks = 4;
    aura.stats.hit = 5.0;
    aura
}

pub fn ephemeral_power() -> Aura {
    let mut aura = Aura::new(EPHEMERAL_POWER, String::from("Ephemeral Power"), 15.0);
    aura.stats.sp = 175.0;
    aura
}

pub fn essence_of_sapphiron() -> Aura {
    let mut aura = Aura::new(ESSENCE_OF_SAPPHIRON, String::from("Essence of Sapphiron"), 20.0);
    aura.stats.sp = 130.0;
    aura
}

pub fn evocation() -> Aura {
    Aura::new(EVOCATION, String::from("Evocation"), 8.0)
}

pub fn fire_vulnerability() -> Aura {
    let mut aura = Aura::new(FIRE_VULNERABILITY, String::from("Fire Vulnerability"), 30.0);
    aura.max_stacks = 5;
    aura
}

pub fn innervate() -> Aura {
    Aura::new(INNERVATE, String::from("Innervate"), 20.0)
}

pub fn mana_tide() -> Aura {
    Aura::new(MANA_TIDE, String::from("Mana Tide"), 12.0)
}

pub fn mind_quickening() -> Aura {
    Aura::new(MIND_QUICKENING, String::from("Mind Quickening"), 20.0)
}

pub fn nat_pagle() -> Aura {
    let mut aura = Aura::new(NAT_PAGLE, String::from("Pagle's Broken Reel"), 15.0);
    aura.stats.hit = 10.0;
    aura
}

pub fn netherwind_focus() -> Aura {
    Aura::new(NETHERWIND_FOCUS, String::from("Netherwind Focus"), 10.0)
}

pub fn obsidian_insight() -> Aura {
    let mut aura = Aura::new(OBSIDIAN_INSIGHT, String::from("Obsidian Insight"), 30.0);
    aura.stats.sp = 50.0;
    aura.stats.spell_penetration = 100.0;
    aura
}

pub fn power_infusion() -> Aura {
    Aura::new(POWER_INFUSION, String::from("Power Infusion"), 15.0)
}

pub fn presence_of_mind() -> Aura {
    Aura::new(PRESENCE_OF_MIND, String::from("Presence of Mind"), 1000.0)
}

pub fn unstable_power() -> Aura {
    let mut aura = Aura::new(UNSTABLE_POWER, String::from("Unstable Power"), 20.0);
    aura.max_stacks = 12;
    aura.stack_increment = -1;
    aura.stack_refresh = false;
    aura.stats.sp = 17.0;
    aura
}

pub fn winters_chill() -> Aura {
    let mut aura = Aura::new(WINTERS_CHILL, String::from("Winter's Chill"), 15.0);
    aura.max_stacks = 5;
    aura
}