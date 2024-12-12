use crate::spell::SpellResult;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum LogType {
    #[default]
    None,
    CastStart,
    CastSuccess,
    SpellImpact,
    Mana,
    AuraGain,
    AuraExpire,
    CooldownGain,
    CooldownExpire,
    Debug,
    Wait,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub log_type: LogType,
    pub unit_name: String,
    pub text: String,
    pub t: f64,
    pub mana: f64,
    pub mana_percent: f64,
    pub dps: f64,
    pub total_dps: f64,
    pub value: f64,
    pub value2: f64,
    pub spell_result: SpellResult,
}