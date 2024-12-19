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
            { key: "None", title: "Do nothing" },
            { key: "Sequence", title: "Sequence" },
            { key: "ArcaneMissiles", title: "Cast: Arcane Missiles" },
            { key: "ArcanePower", title: "Cast: Arcane Power", talent: "arcane_power" },
            { key: "Berserking", title: "Cast: Berserking", race: "Troll" },
            { key: "ColdSnap", title: "Cast: Cold Snap", talent: "cold_snap" },
            { key: "Combustion", title: "Cast: Combustion", talent: "combustion" },
            { key: "Evocation", title: "Cast: Evocation" },
            { key: "Fireball", title: "Cast: Fireball" },
            { key: "FireBlast", title: "Cast: Fire Blast" },
            { key: "Frostbolt", title: "Cast: Frostbolt" },
            { key: "PresenceOfMind", title: "Cast: Presence of Mind", talent: "presence_of_mind" },
            { key: "Pyroblast", title: "Cast: Pyroblast", talent: "pyroblast" },
            { key: "Scorch", title: "Cast: Scorch" },
            { key: "ManaGem", title: "Use: Mana Gem" },
            { key: "ManaPotion", title: "Use: Mana Potion" },
            { key: "CelestialOrb", title: "off_hand", item: items.ids.CELESTIAL_ORB },
            { key: "RobeArchmage", title: "chest", item: items.ids.ROBE_ARCHMAGE },
            { key: "BurstOfKnowledge", title: "trinket", item: items.ids.TRINKET_BURST_OF_KNOWLEDGE },
            { key: "ChromaticInfusion", title: "trinket", item: items.ids.TRINKET_DRACONIC_EMBLEM },
            { key: "EssenceOfSapphiron", title: "trinket", item: items.ids.TRINKET_RESTRAINED_ESSENCE },
            { key: "ObsidianInsight", title: "trinket", item: items.ids.TRINKET_EYE_OF_MOAM },
            { key: "ChaosFire", title: "trinket", item: items.ids.TRINKET_FIRE_RUBY },
            { key: "ArcanePotency", title: "trinket", item: items.ids.TRINKET_HAZZARAH },
            { key: "MindQuickening", title: "trinket", item: items.ids.TRINKET_MQG },
            { key: "NatPagle", title: "trinket", item: items.ids.TRINKET_NAT_PAGLE },
            { key: "EphemeralPower", title: "trinket", item: items.ids.TRINKET_TOEP },
            { key: "ManaInfusion", title: "trinket", item: items.ids.TRINKET_WARMTH_OF_FORGIVENESS },
            { key: "UnstablePower", title: "trinket", item: items.ids.TRINKET_ZHC },
            { key: "Innervate", title: "External: Innervate" },
            { key: "ManaTide", title: "External: Mana Tide", faction: "horde" },
            { key: "PowerInfusion", title: "External: Power Infusion" },
        ];
    },

    apl() {
        return {
            id: common.uuid(),
            type: "apl",
            version: "1.0",
            name: "",
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
            key: "None",
            target_id: 1,
            sequence: [],
        }
    },
    value() {
        return {
            id: common.uuid(),
            value_type: this.value_type.NONE,
            vstr: "",
            vint: 0,
            vfloat: 0,
        }
    },

    getAction(key) {
        let action = this.actions().find(a => a.key == key);
        let defAction = this.action();
        if (!action)
            return defAction;
        for (let key in defAction) {
            if (!action.hasOwnProperty(key))
                action[key] = defAction[key];
        }
        return action;
    },

    isPreset(id) {
        return id.indexOf("preset") === 0;
    },
}