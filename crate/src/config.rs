use crate::common::Race;
use crate::stats::Stats;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Config {
    pub rng_seed: u64,
    pub duration: f64,
    pub duration_variance: f64,
    pub avg_spell_dmg: bool,
    pub race: Race,
    pub talents: Vec<u8>,
    pub player_stats: Stats,
    pub player_level: i32,
    pub target_level: i32,
    pub target_resistance: i32,
    pub targets: i32,
    pub distance: i32,
    pub reaction_time: i32,
    pub pre_cast: bool,

    // Buffs
    pub mage_armor: bool,
    pub mana_spring: bool,
    pub imp_mana_spring: bool,
    pub dmf_dmg: bool,

    // Debuffs
    pub curse_of_elements: bool,
    pub curse_of_shadows: bool,
}