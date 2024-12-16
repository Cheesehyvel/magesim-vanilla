<script setup>
import { ref, computed } from "vue";
import _ from "lodash";
import apl from "../apl";
import { mage as talentTree } from "../talents";
import common from "../common";
import items from "../items";

const props = defineProps(["modelValue", "expect", "player"]);
const emits = defineEmits(["update:modelValue"]);

const typeOptions = [
    { value: apl.value_type.CONST, title: "Constant", input: "vfloat" },
    { value: apl.value_type.PLAYER_MANA, title: "Mana", input: "vfloat" },
    { value: apl.value_type.PLAYER_MANA_PERCENT, title: "Mana %", input: "vfloat" },
    { value: apl.value_type.PLAYER_MANA_DEFICIT, title: "Mana deficit", input: "vfloat" },
    { value: apl.value_type.PLAYER_TALENT_COUNT, title: "Talent points", input: "talent", provides: "vfloat" },
    { value: apl.value_type.PLAYER_COOLDOWN_EXISTS, title: "Cooldown active", input: "cooldown", provides: "bool" },
    { value: apl.value_type.PLAYER_COOLDOWN_REACT, title: "Cooldown active (w/ reaction time)", input: "cooldown", provides: "bool" },
    { value: apl.value_type.PLAYER_COOLDOWN_DURATION, title: "Cooldown duration", input: "cooldown", provides: "vfloat" },
    { value: apl.value_type.PLAYER_AURA_EXISTS, title: "Buff active", input: "buff", provides: "bool" },
    { value: apl.value_type.PLAYER_AURA_REACT, title: "Buff active (w/ reaction time)", input: "buff", provides: "bool" },
    { value: apl.value_type.PLAYER_AURA_STACKS, title: "Buff stacks", input: "buff", provides: "vfloat" },
    { value: apl.value_type.PLAYER_AURA_DURATION, title: "Buff duration", input: "buff", provides: "vfloat" },
    { value: apl.value_type.TARGET_AURA_EXISTS, title: "Debuff active", input: "debuff", provides: "bool" },
    { value: apl.value_type.TARGET_AURA_REACT, title: "Debuff active (w/ reaction time)", input: "debuff", provides: "bool" },
    { value: apl.value_type.TARGET_AURA_STACKS, title: "Debuff stacks", input: "debuff", provides: "vfloat" },
    { value: apl.value_type.TARGET_AURA_DURATION, title: "Debuff duration", input: "debuff", provides: "vfloat" },
    { value: apl.value_type.SPELL_TRAVEL_TIME, title: "Spell travel time", input: "spell", provides: "vfloat" },
    { value: apl.value_type.SPELL_CAST_TIME, title: "Spell cast time", input: "spell", provides: "vfloat" },
    { value: apl.value_type.SPELL_TRAVEL_CAST_TIME, title: "Spell cast + travel time", input: "spell", provides: "vfloat" },
    { value: apl.value_type.SPELL_MANA_COST, title: "Spell mana cost", input: "spell", provides: "vfloat" },
    { value: apl.value_type.SPELL_CAN_CAST, title: "Can cast spell", input: "spell", provides: "bool" },
    { value: apl.value_type.SIM_TIME, title: "Current time", provides: "vfloat" },
    { value: apl.value_type.SIM_TIME_PERCENT, title: "Current time %", provides: "vfloat" },
    { value: apl.value_type.SIM_DURATION, title: "Remaining duration", provides: "vfloat" },
    { value: apl.value_type.SIM_DISTANCE, title: "Target distance", provides: "vfloat" },
    { value: apl.value_type.SIM_REACTION_TIME, title: "Reaction time", provides: "vfloat" },
    { value: apl.value_type.SIM_TARGET_LEVEL, title: "Target level", provides: "vfloat" },
];
const expectedTypeOptions = computed(() => {
    return typeOptions.filter((opt) => {
        if (!opt.input)
            return true;
        if (!props.expect)
            return false;
        if (opt.hasOwnProperty("provides"))
            return opt.provides == props.expect;
        return opt.input == props.expect;
    });
});
const type = computed(() => {
    return typeOptions.find(t => t.value == props.modelValue.value_type);
});

const talentNames = talentTree.trees.reduce((a, b) => { return [...a, ...b.talents.rows.flat()]; }, []).map(t => t.name);
const playerItems = computed(() => {
    return _.values(props.player.loadout).map(i => i.item_id);
});
const filterOptions = (options) => {
    options = options.filter(opt => {
        if (opt.hasOwnProperty("race") && opt.race != props.player.race)
            return false;
        if (opt.hasOwnProperty("faction")) {
            if (opt.faction == "alliance" && ["Gnome", "Human"].indexOf(props.player.race) == -1)
                return false;
            if (opt.faction == "horde" && ["Undead", "Troll"].indexOf(props.player.race) == -1)
                return false;
        }
        if (opt.hasOwnProperty("talent")) {
            let index = talentNames.indexOf(opt.talent);
            if (index == -1 || !props.player.talents[index])
                return false;
        }
        if (opt.hasOwnProperty("item") && playerItems.value.indexOf(opt.item) == -1)
            return false;
        return true;
    });

    for (let option of options) {
        if (option.item) {
            let item = items.gear[option.title].find(i => i.id == option.item);
            if (item)
                option.title = _.upperFirst(option.title)+": "+item.title;
        }
    }

    return options;
};
const talentOptions = computed(() => {
    let options = [];
    for (let tree of talentTree.trees) {
        let talents = tree.talents.rows.flat();
        for (let t=0; t<talents.length; t++)
            options.push({value: t, title: _.upperFirst(talents[t].name.split("_").join(" "))});
    }
    return options;
});
const spellOptions = computed(() => {
    let options = [
        { value: "None", title: "None" },
        { value: "ArcaneMissiles", title: "Arcane Missiles" },
        { value: "ArcanePower", title: "Arcane Power", talent: "arcane_power" },
        { value: "Berserking", title: "Berserking", race: "Troll" },
        { value: "ColdSnap", title: "Cold Snap", talent: "cold_snap" },
        { value: "Combustion", title: "Combustion", talent: "combustion" },
        { value: "Evocation", title: "Evocation" },
        { value: "Fireball", title: "Fireball" },
        { value: "FireBlast", title: "Fire Blast" },
        { value: "Frostbolt", title: "Frostbolt" },
        { value: "PresenceOfMind", title: "Presence of Mind", talent: "presence_of_mind" },
        { value: "Pyroblast", title: "Pyroblast", talent: "pyroblast" },
        { value: "Scorch", title: "Scorch" },
        { value: "ManaGem", title: "Mana Gem" },
        { value: "ManaPotion", title: "Mana Potion" },
        { value: "CelestialOrb", title: "off_hand", item: items.ids.CELESTIAL_ORB },
        { value: "RobeArchmage", title: "chest", item: items.ids.ROBE_ARCHMAGE },
        { value: "BurstOfKnowledge", title: "trinket", item: items.ids.TRINKET_BURST_OF_KNOWLEDGE },
        { value: "ChromaticInfusion", title: "trinket", item: items.ids.TRINKET_DRACONIC_EMBLEM },
        { value: "EssenceOfSapphiron", title: "trinket", item: items.ids.TRINKET_RESTRAINED_ESSENCE },
        { value: "ObsidianInsight", title: "trinket", item: items.ids.TRINKET_EYE_OF_MOAM },
        { value: "ChaosFire", title: "trinket", item: items.ids.TRINKET_FIRE_RUBY },
        { value: "ArcanePotency", title: "trinket", item: items.ids.TRINKET_HAZZARAH },
        { value: "MindQuickening", title: "trinket", item: items.ids.TRINKET_MQG },
        { value: "NatPagle", title: "trinket", item: items.ids.TRINKET_NAT_PAGLE },
        { value: "EphemeralPower", title: "trinket", item: items.ids.TRINKET_TOEP },
        { value: "ManaInfusion", title: "trinket", item: items.ids.TRINKET_WARMTH_OF_FORGIVENESS },
        { value: "UnstablePower", title: "trinket", item: items.ids.TRINKET_ZHC },
    ];
    return filterOptions(options);
});
const cooldownOptions = computed(() => {
    let options = [
        { value: 0, title: "None" },
        { value: common.cooldowns.ARCANE_POWER, title: "Arcane Power", talent: "arcane_power" },
        { value: common.cooldowns.BERSERKING, title: "Berserking", race: "Troll" },
        { value: common.cooldowns.COMBUSTION, title: "Combustion", talent: "combustion" },
        { value: common.cooldowns.PRESENCE_OF_MIND, title: "Presence of Mind", talent: "presence_of_mind" },
        { value: common.cooldowns.MANA_GEM, title: "Mana Gem" },
        { value: common.cooldowns.MANA_POTION, title: "Mana Potion" },
        { value: common.cooldowns.CELESTIAL_ORB, title: "off_hand", item: items.ids.CELESTIAL_ORB },
        { value: common.cooldowns.ROBE_ARCHMAGE, title: "chest", item: items.ids.ROBE_ARCHMAGE },
        { value: common.cooldowns.BURST_OF_KNOWLEDGE, title: "trinket", item: items.ids.TRINKET_BURST_OF_KNOWLEDGE },
        { value: common.cooldowns.BLUE_DRAGON, title: "trinket", item: items.ids.TRINKET_BLUE_DRAGON },
        { value: common.cooldowns.CHROMATIC_INFUSION, title: "trinket", item: items.ids.TRINKET_DRACONIC_EMBLEM },
        { value: common.cooldowns.ESSENCE_OF_SAPPHIRON, title: "trinket", item: items.ids.TRINKET_RESTRAINED_ESSENCE },
        { value: common.cooldowns.OBSIDIAN_INSIGHT, title: "trinket", item: items.ids.TRINKET_EYE_OF_MOAM },
        { value: common.cooldowns.CHAOS_FIRE, title: "trinket", item: items.ids.TRINKET_FIRE_RUBY },
        { value: common.cooldowns.ARCANE_POTENCY, title: "trinket", item: items.ids.TRINKET_HAZZARAH },
        { value: common.cooldowns.MIND_QUICKENING, title: "trinket", item: items.ids.TRINKET_MQG },
        { value: common.cooldowns.NAT_PAGLE, title: "trinket", item: items.ids.TRINKET_NAT_PAGLE },
        { value: common.cooldowns.EPHEMERAL_POWER, title: "trinket", item: items.ids.TRINKET_TOEP },
        { value: common.cooldowns.MANA_INFUSION, title: "trinket", item: items.ids.TRINKET_WARMTH_OF_FORGIVENESS },
        { value: common.cooldowns.UNSTABLE_POWER, title: "trinket", item: items.ids.TRINKET_ZHC },
    ];
    return filterOptions(options);
});
const buffOptions = computed(() => {
    let options = [
        { value: 0, title: "None" },
        { value: common.auras.ARCANE_POWER, title: "Arcane Power", talent: "arcane_power" },
        { value: common.auras.BERSERKING, title: "Berserking", race: "Troll" },
        { value: common.auras.CLEARCAST, title: "Clearcasting", talent: "arcane_concentration" },
        { value: common.auras.COMBUSTION, title: "Combustion", talent: "combustion" },
        { value: common.auras.INNERVATE, title: "Innervate" },
        { value: common.auras.PRESENCE_OF_MIND, title: "Presence of Mind", talent: "presence_of_mind" },
        { value: common.auras.POWER_INFUSION, title: "Power Infusion" },
        { value: common.auras.BURST_OF_KNOWLEDGE, title: "trinket", item: items.ids.TRINKET_BURST_OF_KNOWLEDGE },
        { value: common.auras.BLUE_DRAGON, title: "trinket", item: items.ids.TRINKET_BLUE_DRAGON },
        { value: common.auras.CHROMATIC_INFUSION, title: "trinket", item: items.ids.TRINKET_DRACONIC_EMBLEM },
        { value: common.auras.ESSENCE_OF_SAPPHIRON, title: "trinket", item: items.ids.TRINKET_RESTRAINED_ESSENCE },
        { value: common.auras.OBSIDIAN_INSIGHT, title: "trinket", item: items.ids.TRINKET_EYE_OF_MOAM },
        { value: common.auras.CHAOS_FIRE, title: "trinket", item: items.ids.TRINKET_FIRE_RUBY },
        { value: common.auras.ARCANE_POTENCY, title: "trinket", item: items.ids.TRINKET_HAZZARAH },
        { value: common.auras.MIND_QUICKENING, title: "trinket", item: items.ids.TRINKET_MQG },
        { value: common.auras.NAT_PAGLE, title: "trinket", item: items.ids.TRINKET_NAT_PAGLE },
        { value: common.auras.EPHEMERAL_POWER, title: "trinket", item: items.ids.TRINKET_TOEP },
        { value: common.auras.UNSTABLE_POWER, title: "trinket", item: items.ids.TRINKET_ZHC },
    ];
    return filterOptions(options);
});
const debuffOptions = computed(() => {
    return [
        { value: 0, title: "None" },
        { value: common.auras.FIRE_VULNERABILITY, title: "Fire Vulnerability" },
        { value: common.auras.WINTERS_CHILL, title: "Winter's Chill" },
    ];
});
const intOptions = computed(() => {
    if (!type.value)
        return [];
    if (type.value.input == "talent")
        return talentOptions.value;
    if (type.value.input == "buff")
        return buffOptions.value;
    if (type.value.input == "debuff")
        return debuffOptions.value;
    if (type.value.input == "cooldown")
        return cooldownOptions.value;
    return [];
});
const strOptions = computed(() => {
    if (!type.value)
        return [];
    if (type.value.input == "spell")
        return spellOptions.value;
    return [];
});


/*
 * Update
 */
const changed = () => {
    emits("update:modelValue", props.modelValue);
};
</script>

<template>
    <div class="apl-value">
        <select-simple
            v-model="props.modelValue.value_type"
            :options="expectedTypeOptions"
            :fill-missing="true"
            @input="changed"
        />
        <div class="input" v-if="type && type.input == 'vfloat'">
            <input type="text" v-model.number="props.modelValue.vfloat" @input="changed" size="4">
        </div>
        <select-simple
            v-if="intOptions.length"
            v-model="props.modelValue.vint"
            :options="intOptions"
            :fill-missing="true"
            @input="changed"
        />
        <select-simple
            v-if="strOptions.length"
            v-model="props.modelValue.vstr"
            :options="strOptions"
            :fill-missing="true"
            @input="changed"
        />
    </div>
</template>