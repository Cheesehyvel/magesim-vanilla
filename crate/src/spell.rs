use crate::common::School;
use serde::{Serialize, Deserialize};

pub const ARCANE_MISSILES: i32 = 4145;
pub const ARCANE_POWER: i32 = 12042;
pub const BERSERKING: i32 = 20554;
pub const COLD_SNAP: i32 = 11958;
pub const COMBUSTION: i32 = 29977;
pub const EVOCATION: i32 = 12051;
pub const FIREBALL: i32 = 8400;
pub const FIREBALL_DOT: i32 = -8400; // fake id
pub const FIRE_BLAST: i32 = 10199;
pub const FIRE_VULNERABILITY: i32 = 22959;
pub const FROSTBOLT: i32 = 7322;
pub const IGNITE: i32 = 12848;
pub const INNERVATE: i32 = 29166;
pub const MANA_TIDE: i32 = 17359;
pub const POWER_INFUSION: i32 = 10060;
pub const PRESENCE_OF_MIND: i32 = 12043;
pub const PYROBLAST: i32 = 11366;
pub const PYROBLAST_DOT: i32 = -11366; // fake id
pub const SCORCH: i32 = 2948;
// Trinkets
pub const ARCANE_POTENCY: i32 = 24544;
pub const BURST_OF_KNOWLEDGE: i32 = 15646;
pub const CHAOS_FIRE: i32 = 24389;
pub const CHROMATIC_INFUSION: i32 = 27675;
pub const EPHEMERAL_POWER: i32 = 23271;
pub const ESSENCE_OF_SAPPHIRON: i32 = 28779;
pub const MANA_INFUSION: i32 = 28760;
pub const MIND_QUICKENING: i32 = 23723;
pub const NAT_PAGLE: i32 = 24610;
pub const OBSIDIAN_INSIGHT: i32 = 26166;
pub const UNSTABLE_POWER: i32 = 24658;
// Items
pub const CELESTIAL_ORB: i32 = 9253;
pub const ENGULFING_SHADOWS: i32 = 27860;
pub const MANA_GEM: i32 = 10058;
pub const MANA_POTION: i32 = 17531;
pub const ROBE_ARCHMAGE: i32 = 18385;

#[derive(Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellResult {
    #[default]
    None,
    Hit,
    Crit,
    Miss,
    Pending,
}

#[derive(Clone)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub school: School,
    pub mana_cost: f64,
    pub cast_time: f64,
    pub min_dmg: f64,
    pub max_dmg: f64,
    pub coeff: f64,
    pub cooldown: f64,
    pub gcd: f64,
    pub speed: f64,
    pub rank: i32,
    pub is_channeled: bool,
    pub is_dot: bool,
    pub is_proc: bool,
    pub is_binary: bool,
    pub is_aoe: bool,
    pub is_trigger: bool,
    pub is_dynamic: bool,
    pub can_proc: bool,
    pub can_miss: bool,
    pub can_crit: bool,
    pub tick_refresh: bool,
    pub ticks: u8,
    pub t_interval: f64,

    pub this_mana_cost: f64,
    pub this_cast_time: f64,
    pub this_tick: u8,
}

impl Spell {
    pub fn new(id: i32, name: String, school: School) -> Self {
        Self {
            id,
            name,
            school,
            mana_cost: 0.0,
            cast_time: 0.0,
            min_dmg: 0.0,
            max_dmg: 0.0,
            coeff: 0.0,
            cooldown: 0.0,
            gcd: 1.5,
            speed: 0.0,
            rank: 1,
            is_channeled: false,
            is_dot: false,
            is_proc: false,
            is_binary: false,
            is_aoe: false,
            is_trigger: false,
            is_dynamic: false,
            can_proc: true,
            can_miss: true,
            can_crit: true,
            tick_refresh: true,
            ticks: 0,
            t_interval: 0.0,

            this_mana_cost: 0.0,
            this_cast_time: 0.0,
            this_tick: 0,
        }
    }

    pub fn travel_time(&self, distance: f64) -> f64 {
        if self.speed == 0.0 {
            return 0.0;
        }

        (distance / self.speed).max(0.0)
    }
}

#[derive(Clone)]
pub struct SpellInstance {
    pub spell: Spell,
    pub result: SpellResult,
    pub dmg: f64,
    pub resist: f64,
    pub tick: u8,
}

impl SpellInstance {
    pub fn new(spell: Spell) -> Self {
        Self {
            spell,
            result: SpellResult::None,
            dmg: 0.0,
            resist: 0.0,
            tick: 0,
        }
    }
}


/*
 * Defined spells
 */

// Arcane Missiles
pub fn arcane_missiles() -> Spell {
    arcane_missiles_ranked(8)
}
pub fn arcane_missiles_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(ARCANE_MISSILES, String::from("Arcane Missiles"), School::Arcane);

    spell.rank = rank;
    spell.coeff = 0.24;
    spell.cast_time = 5.0;
    spell.is_channeled = true;
    spell.is_dynamic = true;
    spell.ticks = 5;
    spell.speed = 20.0;

    match rank {
        8 => {
            spell.min_dmg = 196.0;
            spell.max_dmg = 196.0;
            spell.mana_cost = 595.0;
        }
        7 => {
            spell.min_dmg = 155.0;
            spell.max_dmg = 155.0;
            spell.mana_cost = 500.0;
        }
        _ => {
            spell.min_dmg = 26.0;
            spell.max_dmg = 26.0;
            spell.mana_cost = 85.0;
            spell.coeff = 0.132;
        }
    }

    spell
}

// Arcane Potency - Hazza'rah's Charm of Magic
pub fn arcane_potency() -> Spell {
    let mut spell = Spell::new(ARCANE_POTENCY, String::from("Arcane Potency"), School::Arcane);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Arcane Power
pub fn arcane_power() -> Spell {
    let mut spell = Spell::new(ARCANE_POWER, String::from("Arcane Power"), School::Arcane);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Berserking
pub fn berserking() -> Spell {
    let mut spell = Spell::new(BERSERKING, String::from("Berserking"), School::Physical);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Burst of Knowledge
pub fn burst_of_knowledge() -> Spell {
    let mut spell = Spell::new(BURST_OF_KNOWLEDGE, String::from("Burst of Knowledge"), School::Arcane);

    spell.cooldown = 900.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Celestial orb
pub fn celestial_orb() -> Spell {
    let mut spell = Spell::new(CELESTIAL_ORB, String::from("Celestial Orb"), School::Arcane);

    spell.cooldown = 1800.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Chaos Fire - Fire Ruby trinket
pub fn chaos_fire() -> Spell {
    let mut spell = Spell::new(CHAOS_FIRE, String::from("Chaos Fire"), School::Fire);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Chromatic Infusion - Draconic Infused Emblem
pub fn chromatic_infusion() -> Spell {
    let mut spell = Spell::new(CHROMATIC_INFUSION, String::from("Chromatic Infusion"), School::Arcane);

    spell.cooldown = 75.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Cold Snap
pub fn cold_snap() -> Spell {
    let mut spell = Spell::new(COLD_SNAP, String::from("Cold Snap"), School::Frost);

    spell.cooldown = 600.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Combustion
pub fn combustion() -> Spell {
    let mut spell = Spell::new(COMBUSTION, String::from("Combustion"), School::Fire);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Ephemeral Power - Talisman of Ephemeral Power
pub fn ephemeral_power() -> Spell {
    let mut spell = Spell::new(EPHEMERAL_POWER, String::from("Ephemeral Power"), School::Arcane);

    spell.cooldown = 90.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Essence of Sapphiron - The Restrained Essence of Sapphiron
pub fn essence_of_sapphiron() -> Spell {
    let mut spell = Spell::new(ESSENCE_OF_SAPPHIRON, String::from("Essence of Sapphiron"), School::Arcane);

    spell.cooldown = 120.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Evocation
pub fn evocation() -> Spell {
    let mut spell = Spell::new(EVOCATION, String::from("Evocation"), School::Arcane);

    spell.cast_time = 8.0;
    spell.cooldown = 480.0;
    spell.is_trigger = true;
    spell.is_channeled = true;
    spell.ticks = 1;

    spell
}

// Fireball
pub fn fireball() -> Spell {
    fireball_ranked(12)
}
pub fn fireball_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(FIREBALL, String::from("Fireball"), School::Fire);

    spell.rank = rank;
    spell.coeff = 1.0;
    spell.cast_time = 3.5;
    spell.speed = 24.0;

    match rank {
        12 => {
            spell.min_dmg = 596.0;
            spell.max_dmg = 760.0;
            spell.mana_cost = 410.0;
        }
        11 => {
            spell.min_dmg = 561.0;
            spell.max_dmg = 715.0;
            spell.mana_cost = 395.0;
        }
        _ => {
            spell.min_dmg = 16.0;
            spell.max_dmg = 25.0;
            spell.mana_cost = 30.0;
            spell.coeff = 0.123;
            spell.cast_time = 1.5;
        }
    }

    spell
}
pub fn fireball_dot(rank: i32) -> Spell {
    let mut spell = Spell::new(FIREBALL_DOT, String::from("Fireball (dot)"), School::Fire);

    spell.is_dot = true;
    spell.can_proc = false;
    spell.can_miss = false;
    spell.can_crit = false;
    spell.coeff = 0.0;
    spell.t_interval = 2.0;
    spell.ticks = 4;

    match rank {
        12 => {
            spell.min_dmg = 19.0;
            spell.max_dmg = 19.0;
        }
        11 => {
            spell.min_dmg = 18.0;
            spell.max_dmg = 18.0;
        }
        _ => {
            spell.min_dmg = 2.0;
            spell.max_dmg = 2.0;
        }
    }

    spell
}

// Fire blast
pub fn fire_blast() -> Spell {
    fire_blast_ranked(7)
}
pub fn fire_blast_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(FIRE_BLAST, String::from("Fire Blast"), School::Fire);

    spell.rank = rank;
    spell.coeff = 0.429;
    spell.cooldown = 8.0;

    match rank {
        7 => {
            spell.min_dmg = 446.0;
            spell.max_dmg = 524.0;
            spell.mana_cost = 340.0;
        }
        _ => {
            spell.min_dmg = 27.0;
            spell.max_dmg = 35.0;
            spell.mana_cost = 40.0;
            spell.coeff = 0.204;
        }
    }

    spell
}

// Fire Vulnerabiltiy - Improved Scorch
// We put this as a separate spell because it has a separate chance to miss
pub fn fire_vulnerability() -> Spell {
    let mut spell = Spell::new(FIRE_VULNERABILITY, String::from("Fire Vulnerability"), School::Fire);

    spell.can_crit = false;
    spell.gcd = 0.0;

    spell
}

// Frostbolt
pub fn frostbolt() -> Spell {
    frostbolt_ranked(11)
}
pub fn frostbolt_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(FROSTBOLT, String::from("Frostbolt"), School::Frost);

    spell.rank = rank;
    spell.is_binary = true;
    spell.coeff = 0.814; // 3.0 / 3.5 * 0.95
    spell.cast_time = 3.0;
    spell.speed = 28.0;

    match rank {
        11 => {
            spell.mana_cost = 290.0;
            spell.min_dmg = 515.0;
            spell.max_dmg = 555.0;
        }
        10 => {
            spell.mana_cost = 260.0;
            spell.min_dmg = 440.0;
            spell.max_dmg = 475.0;
        }
        _ => {
            spell.mana_cost = 25.0;
            spell.min_dmg = 18.0;
            spell.max_dmg = 20.0;
            spell.cast_time = 1.5;
            spell.coeff = 0.163;
        }
    }

    spell
}

// Ignite
pub fn ignite(dmg: f64) -> Spell {
    let mut spell = Spell::new(IGNITE, String::from("Ignite"), School::Fire);

    spell.is_dot = true;
    spell.can_proc = false;
    spell.can_miss = false;
    spell.can_crit = false;
    spell.t_interval = 2.0;
    spell.ticks = 2;
    spell.min_dmg = dmg;
    spell.max_dmg = dmg;

    spell
}

// Innervate
pub fn innervate() -> Spell {
    let mut spell = Spell::new(INNERVATE, String::from("Innervate"), School::Nature);

    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Mana Gem
pub fn mana_gem() -> Spell {
    let mut spell = Spell::new(MANA_GEM, String::from("Mana Gem"), School::Arcane);

    spell.cooldown = 120.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Mana Infusion - Warmth of Forgiveness
pub fn mana_infusion() -> Spell {
    let mut spell = Spell::new(MANA_INFUSION, String::from("Mana Infusion"), School::Physical);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Mana Potion
pub fn mana_potion() -> Spell {
    let mut spell = Spell::new(MANA_POTION, String::from("Mana Potion"), School::Physical);

    spell.cooldown = 120.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Mana Tide
pub fn mana_tide() -> Spell {
    let mut spell = Spell::new(MANA_TIDE, String::from("Mana Tide"), School::Nature);

    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Mind Quickening - Mind Quickening Gem
pub fn mind_quickening() -> Spell {
    let mut spell = Spell::new(MIND_QUICKENING, String::from("Mind Quickening"), School::Arcane);

    spell.cooldown = 300.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Nat Pagle's Broken Reel
pub fn nat_pagle() -> Spell {
    let mut spell = Spell::new(NAT_PAGLE, String::from("Pagle's Broken Reel"), School::Arcane);

    spell.cooldown = 75.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Obsidian Insight - Eye of Moam
pub fn obsidian_insight() -> Spell {
    let mut spell = Spell::new(OBSIDIAN_INSIGHT, String::from("Obsidian Insight"), School::Arcane);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Power Infusion
pub fn power_infusion() -> Spell {
    let mut spell = Spell::new(POWER_INFUSION, String::from("Power Infusion"), School::Holy);

    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Presence of Mind
pub fn presence_of_mind() -> Spell {
    let mut spell = Spell::new(PRESENCE_OF_MIND, String::from("Presence of Mind"), School::Arcane);

    spell.cooldown = 180.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Pyroblast
pub fn pyroblast() -> Spell {
    pyroblast_ranked(8)
}
pub fn pyroblast_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(PYROBLAST, String::from("Pyroblast"), School::Fire);

    spell.rank = rank;
    spell.coeff = 1.0;
    spell.cast_time = 6.0;
    spell.speed = 24.0;

    match rank {
        8 => {
            spell.min_dmg = 716.0;
            spell.max_dmg = 890.0;
            spell.mana_cost = 440.0;
        }
        _ => {
            spell.min_dmg = 148.0;
            spell.max_dmg = 195.0;
            spell.mana_cost = 125.0;
        }
    }

    spell
}
pub fn pyroblast_dot(rank: i32) -> Spell {
    let mut spell = Spell::new(PYROBLAST_DOT, String::from("Pyroblast (dot)"), School::Fire);

    spell.is_dot = true;
    spell.can_proc = false;
    spell.can_miss = false;
    spell.can_crit = false;
    spell.coeff = 0.15;
    spell.t_interval = 3.0;
    spell.ticks = 4;

    match rank {
        8 => {
            spell.min_dmg = 67.0;
            spell.max_dmg = 67.0;
        }
        _ => {
            spell.min_dmg = 14.0;
            spell.max_dmg = 14.0;
        }
    }

    spell
}

// Robe of the Archmage
pub fn robe_archmage() -> Spell {
    let mut spell = Spell::new(ROBE_ARCHMAGE, String::from("Robe of the Archmage"), School::Arcane);

    spell.cooldown = 300.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}

// Scorch
pub fn scorch() -> Spell {
    scorch_ranked(7)
}
pub fn scorch_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(SCORCH, String::from("Scorch"), School::Fire);

    spell.rank = rank;
    spell.coeff = 0.429;
    spell.cast_time = 1.5;

    match rank {
        7 => {
            spell.min_dmg = 237.0;
            spell.max_dmg = 280.0;
            spell.mana_cost = 150.0;
        }
        _ => {
            spell.min_dmg = 56.0;
            spell.max_dmg = 69.0;
            spell.mana_cost = 50.0;
        }
    }

    spell
}

// Unstable Power - Zandalarian Hero Charm
pub fn unstable_power() -> Spell {
    let mut spell = Spell::new(UNSTABLE_POWER, String::from("Unstable Power"), School::Physical);

    spell.cooldown = 120.0;
    spell.is_trigger = true;
    spell.gcd = 0.0;

    spell
}