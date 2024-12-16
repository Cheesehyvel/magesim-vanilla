use crate::common;
use crate::aura;
use crate::spell;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AplConditionType {
    #[default]
    None,
    And,
    Or,
    Cmp,
    Not,
    False,
    True,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AplConditionOp {
    #[default]
    None,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AplActionType {
    #[default]
    None,
    Spell,
    Sequence,
    Wait,
    Custom,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AplValueType {
    #[default]
    None,
    Const,
    PlayerMana,
    PlayerManaPercent,
    PlayerManaDeficit,
    PlayerTalentCount,
    PlayerCooldownExists,
    PlayerCooldownReact,
    PlayerCooldownDuration,
    PlayerAuraExists,
    PlayerAuraReact,
    PlayerAuraStacks,
    PlayerAuraDuration,
    TargetAuraExists,
    TargetAuraReact,
    TargetAuraStacks,
    TargetAuraDuration,
    SpellTravelTime,
    SpellCastTime,
    SpellTravelCastTime,
    SpellManaCost,
    SpellCanCast,
    SimTime,
    SimTimePercent,
    SimDuration,
    SimDistance,
    SimReactionTime,
    SimTargetLevel,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AplActionKey {
    #[default]
    None,
    ArcaneMissiles,
    ArcanePotency,
    ArcanePower,
    Berserking,
    BurstOfKnowledge,
    CelestialOrb,
    ChaosFire,
    ChromaticInfusion,
    ColdSnap,
    Combustion,
    EphemeralPower,
    EssenceOfSapphiron,
    Evocation,
    Fireball,
    FireBlast,
    Frostbolt,
    Innervate,
    ManaGem,
    ManaInfusion,
    ManaPotion,
    ManaTide,
    MindQuickening,
    NatPagle,
    ObsidianInsight,
    PowerInfusion,
    PresenceOfMind,
    Pyroblast,
    RobeArchmage,
    Scorch,
    Sequence,
    UnstablePower,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Apl {
    pub items: Vec<AplItem>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AplItem {
    pub condition: AplCondition,
    pub action: AplAction,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AplCondition {
    pub condition_type: AplConditionType,
    pub op: AplConditionOp,
    pub conditions: Vec<AplCondition>,
    pub values: Vec<AplValue>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AplAction {
    pub action_type: AplActionType,
    pub key: AplActionKey,
    pub target_id: i32,
    pub sequence: Vec<AplAction>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AplValue {
    pub value_type: AplValueType,
    pub vstr: String,
    pub vfloat: f64,
    pub vint: i32,
}