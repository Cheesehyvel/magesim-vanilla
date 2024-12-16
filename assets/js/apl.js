import common from "./common";
import items from "./items";

export default {
    condition_type: {
        NONE: "None",
        AND: "And",
        OR: "Or",
        CMP: "Cmp",
        NOT: "Not",
        FALSE: "False",
        TRUE: "True",
    },
    condition_op: {
        NONE: "None",
        EQ: "Eq",
        NEQ: "Neq",
        GT: "Gt",
        GTE: "Gte",
        LT: "Lt",
        LTE: "Lte",
    },
    action_type: {
        NONE: "None",
        SPELL: "Spell",
        SEQUENCE: "Sequence",
        WAIT: "Wait",
        CUSTOM: "Custom",
    },
    value_type: {
        NONE: "None",
        CONST: "Const",

        PLAYER_MANA: "PlayerMana",
        PLAYER_MANA_PERCENT: "PlayerManaPercent",
        PLAYER_MANA_DEFICIT: "PlayerManaDeficit",
        PLAYER_TALENT_COUNT: "PlayerTalentCount",
        PLAYER_COOLDOWN_EXISTS: "PlayerCooldownExists",
        PLAYER_COOLDOWN_REACT: "PlayerCooldownReact",
        PLAYER_COOLDOWN_DURATION: "PlayerCooldownDuration",
        PLAYER_AURA_EXISTS: "PlayerAuraExists",
        PLAYER_AURA_REACT: "PlayerAuraReact",
        PLAYER_AURA_STACKS: "PlayerAuraStacks",
        PLAYER_AURA_DURATION: "PlayerAuraDuration",

        TARGET_AURA_EXISTS: "TargetAuraExists",
        TARGET_AURA_REACT: "TargetAuraReact",
        TARGET_AURA_STACKS: "TargetAuraStacks",
        TARGET_AURA_DURATION: "TargetAuraDuration",

        SPELL_TRAVEL_TIME: "SpellTravelTime",
        SPELL_CAST_TIME: "SpellCastTime",
        SPELL_TRAVEL_CAST_TIME: "SpellTravelCastTime",
        SPELL_MANA_COST: "SpellManaCost",
        SPELL_CAN_CAST: "SpellCanCast",

        SIM_TIME: "SimTime",
        SIM_TIME_PERCENT: "SimTimePercent",
        SIM_DURATION: "SimDuration",
        SIM_DISTANCE: "SimDistance",
        SIM_REACTION_TIME: "SimReactionTime",
        SIM_TARGET_LEVEL: "SimTargetLevel",
    },

    actions() {
        return [
            { key: "None", type: this.action_type.NONE, title: "Do nothing" },
            { key: "Sequence", type: this.action_type.SEQUENCE, title: "Sequence" },
            { key: "ArcaneMissiles", type: this.action_type.SPELL, title: "Cast: Arcane Missiles" },
            { key: "ArcanePower", type: this.action_type.SPELL, title: "Cast: Arcane Power", talent: "arcane_power" },
            { key: "Berserking", type: this.action_type.SPELL, title: "Cast: Berserking", race: "Troll" },
            { key: "ColdSnap", type: this.action_type.SPELL, title: "Cast: Cold Snap", talent: "cold_snap" },
            { key: "Combustion", type: this.action_type.SPELL, title: "Cast: Combustion", talent: "combustion" },
            { key: "Evocation", type: this.action_type.SPELL, title: "Cast: Evocation" },
            { key: "Fireball", type: this.action_type.SPELL, title: "Cast: Fireball" },
            { key: "FireBlast", type: this.action_type.SPELL, title: "Cast: Fire Blast" },
            { key: "Frostbolt", type: this.action_type.SPELL, title: "Cast: Frostbolt" },
            { key: "PresenceOfMind", type: this.action_type.SPELL, title: "Cast: Presence of Mind", talent: "presence_of_mind" },
            { key: "Pyroblast", type: this.action_type.SPELL, title: "Cast: Pyroblast", talent: "pyroblast" },
            { key: "Scorch", type: this.action_type.SPELL, title: "Cast: Scorch" },
            { key: "ManaGem", type: this.action_type.SPELL, title: "Use: Mana Gem" },
            { key: "ManaPotion", type: this.action_type.SPELL, title: "Use: Mana Potion" },
            { key: "CelestialOrb", type: this.action_type.SPELL, title: "off_hand", item: items.ids.CELESTIAL_ORB },
            { key: "RobeArchmage", type: this.action_type.SPELL, title: "chest", item: items.ids.ROBE_ARCHMAGE },
            { key: "BurstOfKnowledge", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_BURST_OF_KNOWLEDGE },
            { key: "ChromaticInfusion", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_DRACONIC_EMBLEM },
            { key: "EssenceOfSapphiron", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_RESTRAINED_ESSENCE },
            { key: "ObsidianInsight", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_EYE_OF_MOAM },
            { key: "ChaosFire", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_FIRE_RUBY },
            { key: "ArcanePotency", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_HAZZARAH },
            { key: "MindQuickening", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_MQG },
            { key: "NatPagle", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_NAT_PAGLE },
            { key: "EphemeralPower", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_TOEP },
            { key: "ManaInfusion", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_WARMTH_OF_FORGIVENESS },
            { key: "UnstablePower", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_ZHC },
            { key: "Innervate", type: this.action_type.SPELL, title: "External: Innervate" },
            { key: "ManaTide", type: this.action_type.SPELL, title: "External: Mana Tide", faction: "horde" },
            { key: "PowerInfusion", type: this.action_type.SPELL, title: "External: Power Infusion" },
        ];
    },

    apl() {
        return {
            id: common.uuid(),
            type: "apl",
            version: "1.0",
            items: [],
        };
    },
    item() {
        return {
            id: common.uuid(),
            status: true,
            condition: this.condition(),
            action: this.action(),
        }
    },
    condition() {
        return {
            id: common.uuid(),
            condition_type: this.condition_type.NONE,
            op: this.condition_op.EQ,
            conditions: [],
            values: [],
        }
    },
    action() {
        return {
            id: common.uuid(),
            action_type: this.action_type.NONE,
            key: "none",
            target_id: 1,
            sequence: [],
        }
    },
    value() {
        return {
            id: common.uuid(),
            value_type: this.value_type.NONE,
            vstr: null,
            vint: 0,
            vfloat: 0,
        }
    },
}