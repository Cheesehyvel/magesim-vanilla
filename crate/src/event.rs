use crate::aura;
use crate::cooldown;
use crate::spell;
use crate::unit::Unit;

#[derive(Default, PartialEq, Eq)]
pub enum EventType {
    #[default]
    None,
    CastStart,
    CastFinish,
    CastSuccess,
    SpellImpact,
    SpellTick,
    ManaRegen,
    ManaGain,
    AuraGain,
    AuraExpire,
    CooldownGain,
    CooldownExpire,
    UnitSpawn,
    UnitDespawn,
    Wait,
    Idle,
}

#[derive(Default)]
pub struct Event {
    pub t: f64,
    pub text: String,
    pub event_type: EventType,
    pub unit_id: i32,
    pub target_id: i32,
    pub mana: f64,
    pub spell: Option<spell::Spell>,
    pub spell_instance: Option<spell::SpellInstance>,
    pub aura: Option<aura::Aura>,
    pub cooldown: Option<cooldown::Cooldown>,
    pub is_main_event: bool,
    pub is_quiet: bool,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            t: 0.0,
            text: String::new(),
            event_type,
            unit_id: 0,
            target_id: 0,
            mana: 0.0,
            spell: None,
            spell_instance: None,
            aura: None,
            cooldown: None,
            is_main_event: true,
            is_quiet: false,
        }
    }
}