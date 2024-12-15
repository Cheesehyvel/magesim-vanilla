import common from "./common";
import items from "./items";

export default {
    condition_type: {
        NONE: 0,
        AND: 1,
        OR: 2,
        CMP: 3,
        NOT: 4,
        FALSE: 5,
        TRUE: 6,
    },
    condition_op: {
        NONE: 0,
        EQ: 1,
        NEQ: 2,
        GT: 3,
        GTE: 4,
        LT: 5,
        LTE: 6,
    },
    action_type: {
        NONE: 0,
        SPELL: 1,
        SEQUENCE: 2,
        WAIT: 3,
        CUSTOM: 4,
    },
    value_type: {
        NONE: 0,
        CONST: 1,

        PLAYER_MANA: 10,
        PLAYER_MANA_PERCENT: 11,
        PLAYER_MANA_DEFICIT: 12,
        PLAYER_TALENT_COUNT: 14,
        PLAYER_COOLDOWN_EXISTS: 15,
        PLAYER_COOLDOWN_REACT: 16,
        PLAYER_COOLDOWN_DURATION: 17,
        PLAYER_AURA_EXISTS: 18,
        PLAYER_AURA_REACT: 19,
        PLAYER_AURA_STACKS: 20,
        PLAYER_AURA_DURATION: 21,

        TARGET_AURA_EXISTS: 30,
        TARGET_AURA_REACT: 31,
        TARGET_AURA_STACKS: 32,
        TARGET_AURA_DURATION: 33,

        SPELL_TRAVEL_TIME: 40,
        SPELL_CAST_TIME: 41,
        SPELL_TRAVEL_CAST_TIME: 42,
        SPELL_MANA_COST: 43,
        SPELL_CAN_CAST: 44,

        SIM_TIME: 50,
        SIM_TIME_PERCENT: 51,
        SIM_DURATION: 52,
        SIM_DURATION_PERCENT: 53,
        SIM_DISTANCE: 54,
        SIM_REACTION_TIME: 55,
        SIM_TARGET_LEVEL: 56,
    },

    actions() {
        return [
            { key: "none", type: this.action_type.NONE, title: "Do nothing" },
            { key: "sequence", type: this.action_type.SEQUENCE, title: "Sequence" },
            { key: "arcane_missiles", type: this.action_type.SPELL, title: "Cast: Arcane Missiles" },
            { key: "arcane_power", type: this.action_type.SPELL, title: "Cast: Arcane Power", talent: "arcane_power" },
            { key: "berserking", type: this.action_type.SPELL, title: "Cast: Berserking", race: "Troll" },
            { key: "cold_snap", type: this.action_type.SPELL, title: "Cast: Cold Snap", talent: "cold_snap" },
            { key: "combustion", type: this.action_type.SPELL, title: "Cast: Combustion", talent: "combustion" },
            { key: "evocation", type: this.action_type.SPELL, title: "Cast: Evocation" },
            { key: "fireball", type: this.action_type.SPELL, title: "Cast: Fireball" },
            { key: "fire_blast", type: this.action_type.SPELL, title: "Cast: Fire Blast" },
            { key: "frostbolt", type: this.action_type.SPELL, title: "Cast: Frostbolt" },
            { key: "presence_of_mind", type: this.action_type.SPELL, title: "Cast: Presence of Mind", talent: "presence_of_mind" },
            { key: "pyroblast", type: this.action_type.SPELL, title: "Cast: Pyroblast", talent: "pyroblast" },
            { key: "scorch", type: this.action_type.SPELL, title: "Cast: Scorch" },
            { key: "mana_gem", type: this.action_type.SPELL, title: "Use: Mana Gem" },
            { key: "mana_potion", type: this.action_type.SPELL, title: "Use: Mana Potion" },
            { key: "celestial_orb", type: this.action_type.SPELL, title: "off_hand", item: items.ids.CELESTIAL_ORB },
            { key: "robe_archmage", type: this.action_type.SPELL, title: "chest", item: items.ids.ROBE_ARCHMAGE },
            { key: "burst_of_knowledge", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_BURST_OF_KNOWLEDGE },
            { key: "chromatic_infusion", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_DRACONIC_EMBLEM },
            { key: "essence_of_sapphiron", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_RESTRAINED_ESSENCE },
            { key: "obsidian_insight", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_EYE_OF_MOAM },
            { key: "chaos_fire", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_FIRE_RUBY },
            { key: "arcane_potency", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_HAZZARAH },
            { key: "mind_quickening", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_MQG },
            { key: "nat_pagle", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_NAT_PAGLE },
            { key: "ephemeral_power", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_TOEP },
            { key: "mana_infusion", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_WARMTH_OF_FORGIVENESS },
            { key: "unstable_power", type: this.action_type.SPELL, title: "trinket", item: items.ids.TRINKET_ZHC },
            { key: "innervate", type: this.action_type.SPELL, title: "External: Innervate" },
            { key: "mana_tide", type: this.action_type.SPELL, title: "External: Mana Tide", faction: "horde" },
            { key: "power_infusion", type: this.action_type.SPELL, title: "External: Power Infusion" },
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
            type: this.condition_type.NONE,
            op: this.condition_op.EQ,
            conditions: [],
            values: [],
        }
    },
    action() {
        return {
            id: common.uuid(),
            type: this.action_type.NONE,
            key: "none",
            target_id: 1,
            sequence: [],
        }
    },
    value() {
        return {
            id: common.uuid(),
            type: this.value_type.NONE,
            vstr: null,
            vint: 0,
            vfloat: 0,
        }
    },
}