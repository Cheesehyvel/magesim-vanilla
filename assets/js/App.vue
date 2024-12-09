<script setup>
import SimContainer from "./sim_container";
import { computed, ref, reactive, watch } from "vue";
import common from "./common";
import items from "./items";
import _ from "lodash";

/**
 * Helpers
 */
const uuid = () => {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function(c) {
        let r = Math.random() * 16 | 0, v = c == "x" ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
};
const css = (str) => {
    return _.kebabCase(str);
};
const raceFaction = (race) => {
    return ["Gnome", "Human"].indexOf(race) == -1 ? "Alliance" : "Horde";
};
const itemUrl = (id) => {
    if (typeof(id) == "object")
        id = id.id;
    if (typeof(id) == "string")
        id = id.replace(":", "&rand=");
    return "https://www.wowhead.com/classic/item="+id;
};
const spellUrl = (id) => {
    return "https://www.wowhead.com/classic/spell="+id;
};

/**
 * Stats
 */
const baseStats = (race) => {
    let stats = common.stats();
    stats.crit = 0.2;

    if (race == "Gnome") {
        stats.int = 132;
        stats.spi = 120;
    }
    else if (race == "Human") {
        stats.int = 125;
        stats.spi = 126;
    }
    else if (race == "Troll") {
        stats.int = 121;
        stats.spi = 121;
    }
    else if (race == "Undead") {
        stats.int = 123;
        stats.spi = 125;
    }

    return stats;
};
const addStats = (a, b) => {
    let stats = common.stats();
    for (let key in stats)
        stats[key] = _.get(a, key, 0) + _.get(b, key, 0);
    return stats;
};

/**
 * Talents
 */
const baseTalents = () => {
    return new Array(50).fill(0);
};
const parseTalents = (url) => {
    let m;
    if (m = url.match(/\/talent-calc\/mage\/([0-9\-]+)/))
        return parseWowheadTalents(m[1]);
    return null;
};
const parseWowheadTalents = (str) => {
    let talents = baseTalents();
    let trees = [0, 16, 32];
    let tree = 0;
    let index = 0;
    let arr = str.split("");
    for (let value of arr) {
        if (value == "-") {
            tree++;
            index = trees[tree];
        }
        else {
            talents[index] = parseInt(value);
            index++;
        }
    }

    return talents;
};


/**
 * Gear / loadout
 */
const loadoutSlotToItemSlot = (slot) => {
    return slot.replace(/[0-9]+/g, "");
};
const getItem = (slot, id) => {
    if (id === undefined) {
        if (!slot)
            return null;
        id = slot;
        for (let key in items.gear) {
            let item = items.gear[key].find(item => item.id == id);
            if (item)
                return item;
        }
        return null;
    }
    else {
        if (!id)
            return null;
        slot = loadoutSlotToItemSlot(slot);
        return items.gear[slot].find(item => item.id == id);
    }
};
const getEnchant = (slot, id) => {
    if (id === undefined) {
        id = slot;
        for (let key in items.enchants) {
            let item = items.enchants[key].find(item => item.id == id || item.enchantment_id == id);
            if (item)
                return item;
        }
        return null;
    }
    else {
        slot = loadoutSlotToItemSlot(slot);
        return items.enchants[slot].find(item => item.id == id || item.enchantment_id == id);
    }
};
const gearUrl = (player, slot) => {
    let itemSlot = player.loadout[slot];
    if (!itemSlot.item_id)
        return null;
    let item = getItem(slot, itemSlot.item_id);
    let url = itemUrl(item.id);

    if (itemSlot.enchant_id) {
        let enchant = getEnchant(itemSlot.enchant_id);
        if (enchant)
            url+= "&ench="+enchant.enchantment_id;
    }

    if (item.set) {
        let pcs = [];
        for (let key in player.loadout) {
            let itm = getItem(key, player.loadout[key].item_id);
            if (_.get(itm, "set") == item.set)
                pcs.push(itm.id);
        }
        if (pcs.length)
            url+= "&pcs="+pcs.join(":");
    }

    return url;
};
const loadoutSlots = () => {
    return [
        "head", "neck", "shoulder", "back", "chest", "wrist",
        "hands", "waist", "legs", "feet",
        "finger1", "finger2", "trinket1", "trinket2",
        "main_hand", "off_hand", "ranged",
    ];
};
const baseLoadout = () => {
    let loadout = {};
    for (let slot of loadoutSlots()) {
        loadout[slot] = {
            item_id: null,
            enchant_id: null,
        };
    }
    return loadout;
};
const loadoutStats = (loadout) => {
    let stats = common.stats();
    for (let slot in loadout) {
        let item = getItem(loadout[slot].item_id);
        if (item) {
            stats = addStats(stats, item);
            let enchant = getEnchant(loadout[slot].enchant_id);
            if (enchant)
                stats = addStats(stats, enchant);
        }
    }
    return stats;
};

/**
 * Config
 */
const defaultConfig = () => {
    return {
        rng_seed: 0,
        duration: 40,
        duration_variance: 0,
        avg_spell_dmg: false,
        target_level: 63,
        target_resistance: 0,
        targets: 1,
        distance: 30,
        reaction_time: 0.3,
        player_delay: 0.1,
        pre_cast: false,
        curse_of_elements: false,
        curse_of_shadows: false,
    };
};

/**
 * Player
 */
const simBasePlayer = () => {
    return {
        name: "Player",
        race: "Gnome",
        talents: parseTalents("https://www.wowhead.com/classic/talent-calc/mage/230005230002-5052000123033151-003"),
        stats: baseStats("Gnome"),
        level: 60,
        mage_armor: true,
        mana_spring: false,
        imp_mana_spring: false,
        dmf_dmg: false,
    };
};
const basePlayer = () => {
    return _.merge(simBasePlayer(), {
        id: uuid(),
        arcane_intellect: true,
        divine_spirit: true,
        motw: true,
        imp_motw: false,
        moonkin_aura: false,
        blessing_of_wisdom: true,
        imp_blessing_of_wisdom: true,
        blessing_of_kings: true,
        atiesh_mage: false,
        atiesh_warlock: false,
        infallible_mind: false,
        songflower: true,
        rallying_cry: true,
        warchiefs_blessing: true,
        spirit_of_zandalar: true,
        elixir_firepower: false,
        elixir_greater_firepower: true,
        elixir_frost_power: true,
        elixir_arcane: false,
        elixir_greater_arcane: true,
        food: common.foods.INT10,
        flask: common.flasks.SUPREME_POWER,
        weapon_oil: common.weapon_oils.BLESSED_WIZARD,
        loadout: baseLoadout(),
        extra_stats: common.stats(),
    });
};
const createPlayer = (name) => {
    let player = basePlayer();
    player.name = name;
    currentRaid.value.players.push(player);
};
const visualStats = (player) => {
    let stats = simStats(player);
    stats.mana+= 1213 + stats.int*15 - 280;
    return stats;
};

/**
 * Raid
 */
const defaultRaid = (name) => {
    return {
        id: uuid(),
        name: "My raid",
        config: defaultConfig(),
        players: [basePlayer()],
    }
};
const loadRaids = () => {
    let raids = window.localStorage.getItem("raids");
    return raids ? JSON.parse(raids) : [defaultRaid()];
};
const saveRaids = (raids) => {
    window.localStorage.setItem("raids", JSON.stringify(raids));
};
const raids = ref(loadRaids());
const deleteRaid = (id) => {
    raids.value = raids.value.filter(raid => raid.id != id);
    saveRaids(raids.value);
};
const currentRaidId = ref(raids.value[0].id);
const currentRaid = computed(() => {
    return raids.value.find(raid => raid.id == currentRaidId.value);
});

/**
 * Settings
 */
const defaultSettings = () => {
    return {
        iterations: 20000,
        threads: navigator.hardwareConcurrency,
    }
};
const loadSettings = () => {
    let settings = window.localStorage.getItem("settings");
    return settings ? JSON.parse(settings) : defaultSettings();
};
const saveSettings = () => {
    window.localStorage.setItem("settings", JSON.stringify(settings));
};
const settings = reactive(loadSettings());

/**
 * Run simulation
 */
const result = ref(null);
const isRunning = ref(false);
const simStats = (player) => {
    let x;
    let faction = raceFaction(player.race);
    let stats = baseStats(player.race);
    stats = addStats(stats, loadoutStats(player.loadout));
    stats = addStats(stats, player.extra_stats);

    if (player.arcane_intellect)
        stats.int+= 31;
    if (player.divine_spirit)
        stats.spi+= 40;
    if (player.moonkin_aura)
        stats.crit+= 3;

    if (player.motw) {
        x = 12;
        if (player.imp_motw)
            x = _.round(x * 1.35);
        stats.int+= x;
        stats.spi+= x;
    }

    if (player.elixir_greater_firepower)
        stats.sp_fire+= 40;
    else if (player.elixir_firepower)
        stats.sp_fire+= 10;
    if (player.elixir_frost_power)
        stats.sp_frost+= 15;
    if (player.elixir_greater_arcane)
        stats.sp_arcane+= 35;
    else if (player.elixir_arcane)
        stats.sp_arcane+= 20;

    if (player.food == common.foods.INT10)
        stats.int+= 10;
    else if (player.food == common.foods.SPIRIT12)
        stats.spi+= 12;
    else if (player.food == common.foods.MP8)
        stats.mp5+= 8;

    if (player.flask == common.flasks.SUPREME_POWER)
        stats.sp+= 150;
    else if (player.flask == common.flasks.DISTILLED_WISDOM)
        stats.mana+= 2000;

    if (player.weapon_oil == common.weapon_oils.BLESSED_WIZARD)
        stats.sp+= 60;
    else if (player.weapon_oil == common.weapon_oils.BRILLIANT_WIZARD) {
        stats.sp+= 36;
        stats.crit+= 1;
    }
    else if (player.weapon_oil == common.weapon_oils.WIZARD)
        stats.sp+= 24;
    else if (player.weapon_oil == common.weapon_oils.BRILLIANT_MANA)
        stats.mp5+= 12;

    if (player.blessing_of_wisdom && faction == "Alliance") {
        x = 30;
        if (player.imp_blessing_of_wisdom)
            x = _.round(x * 1.2);
        stats.mp5+= x;
    }

    if (player.atiesh_mage && player.loadout.main_hand.item_id != items.ids.ATIESH)
        stats.crit+= 2;
    if (player.atiesh_warlock)
        stats.sp+= 33;

    if (player.infallible_mind)
        stats.int+= 25;
    if (player.songflower) {
        stats.crit+= 5;
        stats.int+= 15;
        stats.spi+= 15;
    }
    if (player.rallying_cry)
        stats.crit+= 10;
    if (player.warchiefs_blessing)
        stats.mp5+= 10;

    if (player.spirit_of_zandalar) {
        stats.int*= 1.15;
        stats.spi*= 1.15;
    }
    if (player.race == "Gnome")
        stats.int*= 1.05;
    if (player.race == "Human")
        stats.spi*= 1.05;
    if (player.blessing_of_kings && faction == "Alliance") {
        stats.int*= 1.1;
        stats.spi*= 1.1;
    }

    stats.int = Math.round(stats.int);
    stats.spi = Math.round(stats.spi);

    stats.crit+= stats.int / 59.5;

    return stats;
};
const simConfig = () => {
    let config = _.cloneDeep(currentRaid.value.config);
    config.players = [];
    for (let p of currentRaid.value.players) {
        let player = simBasePlayer();
        for (var key in player)
            player[key] = _.cloneDeep(p[key]);
        player.stats = simStats(p);
        config.players.push(player);
    }
    return config;
};
const runSingle = () => {
    const sc = new SimContainer(settings.threads, 1, simConfig(), r => {
        isRunning.value = false;
        result.value = r;
    }, e => {
        console.error("Error", e);
    });

    isRunning.value = true;
    result.value = null;
    sc.start();
};
const simProgress = reactive({
    dps: 0,
    progress: 0,
});
const runMultiple = () => {
    let iterations = settings.iterations;
    const sc = new SimContainer(settings.threads, settings.iterations, simConfig(), r => {
        isRunning.value = false;
        result.value = r;
    }, e => {
        console.error("Error", e);
    }, p => {
        simProgress.dps = p.avg_dps;
        simProgress.progress = p.iterations / iterations;
    });

    isRunning.value = true;
    result.value = null;
    simProgress.dps = 0;
    simProgress.progress = 0;
    sc.start();
};

/**
 * Combat log
 */
const formatter = (text, key, cl) => {
    let regex = new RegExp(key+"\\[([^\\]]+)\\]", "g");
    return text.replace(regex, "<span class='format-"+cl+"'>$1</span>");
};
const formatTime = (s) => {
    let str = s.toFixed(2);
    return str.length == 4 ? "0"+str : str;
};
const formatLogText = (log) => {
    let text = log.text;
    text = formatter(text, "s", "spell");
    text = formatter(text, "t", "target");
    text = formatter(text, "a", "aura");
    text = formatter(text, "c", "cooldown");
    text = formatter(text, "m", "mana");
    text = text.replace("->", "&#8594;");
    // Remove unknown formatters
    text = text.replace(/[a-z0-9]+\[([^\]]+)\]/g, "$1");
    return text;
};
const formatLogType = (type) => {
    return _.capitalize(_.kebabCase(type).replace(/-/g, " "));
};
const logTypes = ref([
    "CastSuccess", "SpellImpact", "Wait", "AuraGain", "AuraExpire",
]);
const filteredLog = computed(() => {
    if (!result.value)
        return [];
    return result.value.log.filter(log => logTypes.value.indexOf(log.log_type) != -1);
});

/**
 * Watchers
 */
watch(settings, saveSettings, {deep : true});
</script>

<template>
    <div class="app">
        <div style="margin: 20px 0">
            <input type="text" v-model.number="currentRaid.config.duration" style="width: 50px" :disabled="isRunning"> duration<br>
            <input type="text" v-model.number="settings.iterations" style="width: 50px" :disabled="isRunning"> iterations<br>
            <input type="text" v-model.number="settings.threads" style="width: 50px" :disabled="isRunning"> threads<br><br>
            <button class="btn btn-primary" @click="runMultiple" :disabled="isRunning">Run multiple</button>&nbsp;
            <button class="btn btn-secondary" @click="runSingle" :disabled="isRunning">Run single</button>
        </div>

        <template v-if="isRunning">
            <div>Progress: {{ (simProgress.progress * 100).toFixed() }}% ({{ simProgress.dps.toFixed(2) }} dps)</div>
        </template>

        <template v-if="result">
            <div v-if="result.time" style="margin: 0 0 10px;">Completed {{ result.iterations }} in {{ result.time.toFixed(2) }}s</div>
            <template v-if="result.iterations">
                <div><b>Total dps: {{ result.avg_dps.toFixed(2) }} ({{ result.min_dps.toFixed() }} - {{ result.max_dps.toFixed() }})</b></div>
                <div v-for="(player, index) in result.players">
                    <div>Player {{ index+1 }} dps: {{ player.dps.toFixed(2) }}</div>
                </div>
            </template>
            <template v-else>
                <div><b>Total: {{ result.dps.toFixed(2) }} dps ({{ result.dmg }} dmg)</b></div>
                <div v-for="(player, index) in result.players">
                    <div>Player {{ index+1 }}: {{ player.dps.toFixed(2) }} dps ({{ player.dmg }} dmg)</div>
                </div>
                <div class="combat-log">
                    <span><label><input type="checkbox" v-model="logTypes" value="CastStart"> Cast start</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="CastSuccess"> Cast success</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="SpellImpact"> Spell impact</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="AuraGain"> Aura gain</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="AuraExpire"> Aura expire</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="CooldownGain"> Cooldown gain</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="CooldownExpire"> Cooldown expire</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="Mana"> Mana gain/loss</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="Wait"> Wait</label></span>&nbsp;&nbsp;&nbsp;
                    <span><label><input type="checkbox" v-model="logTypes" value="Debug"> Debug</label></span>&nbsp;&nbsp;&nbsp;
                    <br><br>
                    <table>
                        <thead>
                            <tr>
                                <th>Time</th>
                                <th>Mana</th>
                                <th>Unit</th>
                                <th>Type</th>
                                <th>Text</th>
                                <th>Result</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="log in filteredLog" :class="['log-type-'+css(log.log_type)]">
                                <td>{{ formatTime(log.t) }}</td>
                                <td>{{ log.mana.toFixed() }} ({{ log.mana_percent.toFixed() }}%)</td>
                                <td>{{ log.unit_name }}</td>
                                <td>{{ formatLogType(log.log_type) }}</td>
                                <td class="text" v-html="formatLogText(log)"></td>
                                <td>
                                    <span v-if="log.dmg" class="format-dmg" :class="['spell-result-'+css(log.spell_result)]">
                                        {{ log.dmg.toFixed() }}
                                    </span>
                                    <span v-if="log.resist">
                                        (-{{ log.resist.toFixed() }})
                                    </span>
                                    <span v-if="log.spell_result == 'Miss'">
                                        Miss
                                    </span>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </template>
        </template>
    </div>
</template>