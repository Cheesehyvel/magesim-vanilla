use crate::common::School;
use serde::{Serialize, Deserialize};

pub const ARCANE_MISSILES: i32 = 4145;
pub const COLD_SNAP: i32 = 11958;
pub const EVOCATION: i32 = 12051;
pub const FIREBALL: i32 = 8400;
pub const FIREBALL_DOT: i32 = -8400; // fake id
pub const FIRE_BLAST: i32 = 10199;
pub const FROSTBOLT: i32 = 7322;
pub const IGNITE: i32 = 12848;
pub const MANA_RUBY: i32 = 10058;
pub const PYROBLAST: i32 = 11366;
pub const PYROBLAST_DOT: i32 = -11366; // fake id
pub const SCORCH: i32 = 2948;
// Trinkets
pub const CHAOS_FIRE: i32 = 24389;
pub const MANA_INFUSION: i32 = 28760;
// Items
pub const ELECTROMAGNETIC_GIGAFLUX_REACTIVATOR: i32 = 11826;
pub const ROBE_ARCHMAGE: i32 = 18385;
pub const CELESTIAL_ORB: i32 = 9253;
pub const ENGULFING_SHADOWS: i32 = 27860;

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
    pub is_fixed_dmg: bool,
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
            is_fixed_dmg: false,
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

pub fn fireball() -> Spell {
    fireball_ranked(12)
}

pub fn fireball_ranked(rank: i32) -> Spell {
    let mut spell = Spell::new(FIREBALL, String::from("Fireball"), School::Fire);

    // spell.name = format!("Fireball (Rank {})", rank);
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