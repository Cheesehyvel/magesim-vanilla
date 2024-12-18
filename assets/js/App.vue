<script setup>
import SimContainer from "./sim_container";
import { computed, ref, reactive, watch, onMounted, nextTick } from "vue";
import common from "./common";
import icons from "./icons";
import items from "./items";
import aplData from "./apl";
import { mage as talentTree } from "./talents";
import _ from "lodash";
import { compressToEncodedURIComponent, decompressFromEncodedURIComponent } from "lz-string";

/*
 * Helpers
 */
const css = (str) => {
    return _.kebabCase(str);
};
const raceFaction = (race) => {
    return ["Gnome", "Human"].indexOf(race) != -1 ? "Alliance" : "Horde";
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
const convertRace = (from) => {
    if (from == "Gnome")
        return "Troll";
    if (from == "Human")
        return "Undead";
    if (from == "Troll")
        return "Gnome";
    if (from == "Undead")
        return "Human";
    return from;
};
const otherSlot = (slot) => {
    var n = slot.substr(-1);
    n = parseInt(n);
    if (isNaN(n))
        return null;
    n = n == 1 ? 2 : 1;
    return slot.substr(0, slot.length-1)+n;
};
const copyToClipboard = (str) => {
    var el = document.createElement("textarea");
    el.value = str;
    el.style.opacity = 0;
    el.style.position = "absolute";
    el.style.top = 0;
    document.body.appendChild(el);
    el.select();
    document.execCommand("copy")
    document.body.removeChild(el);
};

/*
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
        stats[key] = parseFloat(_.get(a, key, 0)) + parseFloat(_.get(b, key, 0));
    return stats;
};

/*
 * Talents
 */
const baseTalents = () => {
    return new Array(49).fill(0);
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
const talentPresets = () => {
    return [
        { name: "Fire", talents: parseWowheadTalents("230005230002-5052000123033151-003") },
        { name: "Frost Arcane", talents: parseWowheadTalents("2300250310231531--053500030013") },
        { name: "Frost WC", talents: parseWowheadTalents("230045200003--05350013122301051") },
    ]
};


/*
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
    let sets = {};

    for (let slot in loadout) {
        let item = getItem(loadout[slot].item_id);
        if (item) {
            stats = addStats(stats, item);

            let enchant = getEnchant(loadout[slot].enchant_id);
            if (enchant)
                stats = addStats(stats, enchant);

            if (item.set) {
                if (!sets.hasOwnProperty(item.set)) {
                    let set = _.find(items.sets, {id: item.set});
                    sets[item.set] = {
                        set: set,
                        n: 1,
                    };
                }
                else {
                    sets[item.set].n++;
                    if (sets[item.set].set) {
                        let setbonus = _.get(sets[item.set].set, "set"+sets[item.set].n);
                        if (setbonus)
                            stats = addStats(stats, setbonus);
                    }
                }
            }
        }
    }
    return stats;
};
const isItemSpecial = (id) => {
    for (let key in items.ids) {
        if (items.ids[key] == id)
            return true;
    }
    return false;
};

/*
 * APL
 */
const defaultApls = () => {
    let data = [];
    let item, cond;

    let manaCds = [];
    item = aplData.item();
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.GTE;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.PLAYER_MANA_DEFICIT;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 2250;
    item.action = aplData.getAction("ManaPotion");
    manaCds.push(item);
    item = aplData.item();
    item.condition.condition_type = aplData.condition_type.AND;
    cond = aplData.condition();
    cond.condition_type = aplData.condition_type.CMP;
    cond.op = aplData.condition_op.GTE;
    cond.values = [aplData.value(), aplData.value()];
    cond.values[0].value_type = aplData.value_type.PLAYER_MANA_DEFICIT;
    cond.values[1].value_type = aplData.value_type.CONST;
    cond.values[1].vfloat = 1200;
    item.condition.conditions.push(cond);
    cond = aplData.condition();
    cond.condition_type = aplData.condition_type.TRUE;
    cond.values = [aplData.value()];
    cond.values[0].value_type = aplData.value_type.PLAYER_COOLDOWN_EXISTS;
    cond.values[0].vint = common.cooldowns.MANA_POTION;
    item.condition.conditions.push(cond);
    item.action = aplData.getAction("ManaGem");
    manaCds.push(item);
    item = aplData.item();
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.LT;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.PLAYER_MANA_PERCENT;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 10;
    item.action = aplData.getAction("Evocation");
    manaCds.push(item);

    let cds = [
        aplData.getAction("ArcanePower"),
        aplData.getAction("Combustion"),
        aplData.getAction("EssenceOfSapphiron"),
        aplData.getAction("UnstablePower"),
        aplData.getAction("EphemeralPower"),
        aplData.getAction("ChaosFire"),
        aplData.getAction("MindQuickening"),
    ];

    let frost = aplData.apl();
    frost.id = "default-frost";
    frost.name = "Frost";
    item = aplData.item();
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.LT;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.SIM_TIME;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 1.4;
    item.action = aplData.getAction("Sequence");
    item.action.sequence = [
        aplData.getAction("Frostbolt"),
        aplData.getAction("PowerInfusion"),
        ...cds
    ];
    frost.items.push(item);
    frost.items = [...frost.items, ...manaCds];
    item = aplData.item();
    item.action = aplData.getAction("Sequence");
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.GT;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.SIM_TIME;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 90;
    item.action.sequence = cds;
    frost.items.push(item);
    item = aplData.item();
    item.action = aplData.getAction("Frostbolt");
    frost.items.push(item);
    data.push(frost);

    let fire = aplData.apl();
    fire.id = "default-fire";
    fire.name = "Fire";
    item = aplData.item();
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.LT;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.SIM_TIME;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 1.4;
    item.action = aplData.getAction("Sequence");
    item.action.sequence = [
        aplData.getAction("Scorch"),
        aplData.getAction("Scorch"),
        aplData.getAction("Frostbolt"),
        aplData.getAction("PowerInfusion"),
        ...cds.slice(1)
    ];
    fire.items.push(item);
    fire.items = [...fire.items, ...manaCds];
    item = aplData.item();
    item.action = aplData.getAction("Sequence");
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.GT;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.SIM_TIME;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 90;
    item.action.sequence = cds;
    fire.items.push(item);
    item = aplData.item();
    item.action = aplData.getAction("Fireball");
    fire.items.push(item);
    data.push(fire);

    let f2 = _.cloneDeep(fire);
    f2.name = "Fire - delayed combustion"
    f2.id = "default-fire-d1";
    f2.items[0].action.sequence.splice(3, 1); // Remove combustion
    item = aplData.item();
    item.condition.condition_type = aplData.condition_type.CMP;
    item.condition.op = aplData.condition_op.GT;
    item.condition.values = [aplData.value(), aplData.value()];
    item.condition.values[0].value_type = aplData.value_type.SIM_TIME;
    item.condition.values[1].value_type = aplData.value_type.CONST;
    item.condition.values[1].vfloat = 10.0;
    item.action = aplData.getAction("Combustion");
    f2.items.splice(1, 0, item);
    data.push(f2);

    let blank = aplData.apl();
    blank.id = "default-blank";
    blank.name = "Blank";
    data.push(blank);

    return data;
};
const isDefaultApl = (id) => {
    return id.indexOf("default") != -1;
};

/*
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
        curse_of_elements: false,
        curse_of_shadows: false,
        judgement_of_wisdom: false,
    };
};

/*
 * Player
 */
const simDefaultPlayer = () => {
    return {
        name: "Player",
        race: "Undead",
        talents: parseTalents("https://www.wowhead.com/classic/talent-calc/mage/230005230002-5052000123033151-003"),
        stats: baseStats("Undead"),
        level: 60,
        items: [],
        apl: defaultApls()[0],
        mage_armor: true,
        mana_spring: true,
        imp_mana_spring: true,
        dmf_dmg: false,
        soul_revival: false,
        traces_of_silithyst: false,
    };
};
const defaultPlayer = () => {
    return _.merge(simDefaultPlayer(), {
        id: common.uuid(),
        arcane_intellect: true,
        divine_spirit: true,
        motw: true,
        imp_motw: true,
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
        food: common.foods.RUNN_TUM,
        flask: common.flasks.SUPREME_POWER,
        weapon_oil: common.weapon_oils.BRILLIANT_WIZARD,
        loadout: baseLoadout(),
        bonus_stats: common.stats(),
    });
};
const createPlayer = (name) => {
    let player = defaultPlayer();
    player.name = name;
    activeRaid.value.players.push(player);
};
const visualStats = (player) => {
    let stats = simStats(player);
    stats.mana+= 1213 + stats.int*15 - 280;
    return stats;
};
const activePlayerId = ref(null);
const activePlayer = computed(() => {
    if (!activeRaid.value)
        return null;
    return activeRaid.value.players.find(player => player.id == activePlayerId.value);
});

/*
 * Raid
 */
const defaultRaid = (name) => {
    return {
        id: common.uuid(),
        name: "My raid",
        faction: "Horde",
        config: defaultConfig(),
        players: [defaultPlayer()],
        _sync_buffs: false,
    }
};
const loadRaids = () => {
    let raids = window.localStorage.getItem("raids");
    if (raids)
        raids = JSON.parse(raids);

    if (_.isEmpty(raids)) {
        raids = [defaultRaid()];
    }
    else {
        let defRaid = defaultRaid();
        let defPlayer = defaultPlayer();
        // Convert data
        for (let raid of raids) {
            for (let player of raid.players) {
                player.talents.splice(49);
                if (player.hasOwnProperty("extra_stats")) {
                    player.bonus_stats = player.extra_stats;
                    delete player.extra_stats;
                }
                for (let key in defPlayer) {
                    if (!player.hasOwnProperty(key))
                        player[key] = defPlayer[key];
                }
                // Reload preset apl
                if (isDefaultApl(player.apl.id)) {
                    let ap = defaultApls().find(a => a.id == player.apl.id);
                    if (ap)
                        player.apl = ap;
                }
            }
            for (let key in defRaid) {
                if (!raid.hasOwnProperty(key))
                    raid[key] = defRaid[key];
            }
            for (let key in defRaid.config) {
                if (!raid.config.hasOwnProperty(key))
                    raid.config[key] = defRaid.config[key];
            }
        }
    }

    return raids;
};
const saveRaids = (raids) => {
    window.localStorage.setItem("raids", JSON.stringify(raids));
};
const raids = ref(loadRaids());
const deleteRaid = (id) => {
    raids.value = raids.value.filter(raid => raid.id != id);
    if (!raids.value.length)
        raids.value.push(defaultRaid());
    if (settings.raid_id == id)
        settings.raid_id = raids.value[0].id;
    saveRaids(raids.value);
};
const activeRaid = computed(() => {
    return raids.value.find(raid => raid.id == settings.raid_id);
});

/*
 * Settings
 */
const detectedCores = () => {
    return navigator.hardwareConcurrency;
};
const defaultSettings = () => {
    return {
        iterations: 10000,
        threads: detectedCores(),
        raid_id: null,
        normalize_ignite: false,
    }
};
const loadSettings = () => {
    let settings = window.localStorage.getItem("settings");
    if (settings) {
        settings = JSON.parse(settings);
        settings = _.merge(defaultSettings(), settings);
    }
    else {
        settings = defaultSettings();
    }

    if (!settings.raid_id || !raids.value.find(raid => raid.id == settings.raid_id))
        settings.raid_id = raids.value[0].id;

    return settings;
};
const saveSettings = () => {
    window.localStorage.setItem("settings", JSON.stringify(settings));
};
const settings = reactive(loadSettings());

/*
 * Run simulation
 */
const result = ref(null);
const isRunning = ref(false);
const simStats = (player) => {
    let x;
    let faction = raceFaction(player.race);
    let stats = baseStats(player.race);
    stats = addStats(stats, loadoutStats(player.loadout));
    stats = addStats(stats, player.bonus_stats);

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

    if (player.food == common.foods.RUNN_TUM)
        stats.int+= 10;
    else if (player.food == common.foods.WELL_FED)
        stats.spi+= 12;
    else if (player.food == common.foods.NIGHTFIN)
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
const simLoadoutItems = (loadout) => {
    let arr = [];
    for (let slot in loadout) {
        let item = getItem(slot, loadout[slot].item_id);
        if (item) {
            if (item.id && isItemSpecial(item.id))
                arr.push(item.id);
            if (item.set)
                arr.push(item.set);
        }
    }
    return arr;
};
const simApl = (apl) => {
    apl = _.cloneDeep(apl);
    for (let i=0; i<apl.items.length; i++) {
        if (!apl.items[i].status) {
            apl.items.splice(i, 1);
            i--;
        }
    }
    return apl;
};
const simConfig = () => {
    let config = _.cloneDeep(activeRaid.value.config);
    for (let key in config) {
        if (key.substr(0, 1) == "_")
            delete config[key];
    }

    if (config.targets < 1)
        config.targets = 1;

    if (activeRaid.value.faction == "Horde" && config.judgement_of_wisdom)
        config.judgement_of_wisdom = false;

    config.players = [];
    for (let p of activeRaid.value.players) {
        let player = simDefaultPlayer();
        for (var key in player)
            player[key] = _.cloneDeep(p[key]);
        player.stats = simStats(p);
        player.items = simLoadoutItems(p.loadout);
        player.apl = simApl(p.apl);
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
        simProgress.dps = p.dps;
        simProgress.progress = p.iterations / iterations;
    });

    isRunning.value = true;
    result.value = null;
    simProgress.dps = 0;
    simProgress.progress = 0;
    sc.start();
};

/*
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
    text = text.replace("!=", "&ne;");
    // Remove unknown formatters
    text = text.replace(/[a-z0-9]+\[([^\]]+)\]/g, "$1");
    return text;
};
const formatLogType = (type) => {
    return _.capitalize(_.kebabCase(type).replace(/-/g, " "));
};
const filterPlayer = ref(null);
const filterPlayerOptions = computed(() => {
    if (!result.value)
        return [];
    let options = [];
    for (let player of result.value.players)
        options.push({value: player.name, title: player.name});
    return options;
});
const filteredLog = computed(() => {
    if (!result.value)
        return [];
    let log = result.value.log.filter(log => log.log_type != "Debug");
    if (filterPlayer.value)
        log = log.filter(log => log.unit_name == filterPlayer.value);
    return log;
});


/*
 * Confirmation
 */
const confirmSpotlight = ref();
const confirmation = ref({});
const confirm = (options) => {
    let defaults = {
        text: "Are you sure?",
        confirm: "Yes",
        abort: "No",
        continue: () => {},
    };
    confirmation.value = _.merge(defaults, options);
    confirmSpotlight.value.open();

    return new Promise((resolve, reject) => {
        confirmation.value.continue = resolve;
    });
};
const confirmationContinue = () => {
    confirmation.value.continue();
    confirmSpotlight.value.close();
};
const confirmationCancel = () => {
    confirmSpotlight.value.close();
};

/*
 * Alert
 */
const alertSpotlight = ref();
const alerter = reactive({
    text: "",
});
const alert = (text) => {
    alerter.text = text;
    alertSpotlight.value.open();
};
const alertClose = () => {
    alertSpotlight.value.close();
};

/*
 * Notification
 */
const notifications = ref([]);
const notify = (obj) => {
    if (!obj.id)
        obj.id = common.uuid();
    notifications.value.push(obj);
    if (obj.timer && obj.timer > 0) {
        setTimeout(() => {
            notifications.value = notifications.value.filter(n => n.id != obj.id);
        }, obj.timer);
    }
};

/*
 * Raid UI
 */
const raidSelectOpen = ref(false);
const confirmDeleteRaid = (raid) => {
    raidSelectOpen.value = false;
    confirm({
        text: "Are you sure you want to delete "+raid.name+"?",
        confirm: "Delete",
        abort: "Cancel",
    }).then(() => {
        deleteRaid(raid.id);
    });
};
const selectRaid = (id) => {
    settings.raid_id = id;
    raidSelectOpen.value = false;
};
const raidEdit = ref();
const raidModel = ref(defaultRaid());
const raidModelCopy = ref(null);
const raidCopyOptions = computed(() => {
    if (!activeRaid.value)
        return [];
    let options = [];
    for (let raid of raids.value) {
        if (raid.id == raidModel.value.id)
            continue;
        options.push({value: raid.id, title: raid.name});
    }
    return options;
});
const createRaidOpen = () => {
    raidModel.value = defaultRaid();
    raidModel.value.name = "";
    raidSelectOpen.value = false;
    raidEdit.value.open(true);
};
const updateRaid = () => {
    raidEdit.value.close();
    let raid = _.cloneDeep(raidModel.value);

    if (raidModelCopy.value) {
        let copy = raids.value.find(r => r.id == raidModelCopy.value);
        if (copy) {
            raid = _.cloneDeep(copy);
            raid.id = raidModel.value.id;
            raid.name = raidModel.value.name;
            raid.faction = raidModel.value.faction;
            if (raid.faction != copy.faction) {
                for (let player of raid.players)
                    player.race = convertRace(player.race);
            }
        }
    }

    let index = raids.value.findIndex(r => r.id == raid.id);
    if (index != -1) {
        if (raids.value[index].faction != raid.faction) {
            for (let player of raid.players)
                player.race = convertRace(player.race);
        }
        raids.value[index] = raid;
    }
    else {
        raids.value.push(raid);
        settings.raid_id = raid.id;
    }
    raids.value = _.sortBy(raids.value, "name");
};
const factionOptions = [
    { value: "Alliance", title: "Alliance" },
    { value: "Horde", title: "Horde" },
];

/*
 * Player UI
 */
const playerStats = ref(common.stats());
const playerEdit = ref();
const playerImport = ref();
const playerModel = ref(defaultPlayer());
const playerModelCopy = ref(null);
const playerImportConfig = ref(true);
const playerImportLoadout = ref(true);
const selectPlayer = (id) => {
    activePlayerId.value = id;
};
const confirmDeletePlayer = (player) => {
    raidSelectOpen.value = false;
    confirm({
        text: "Are you sure you want to delete "+player.name+"?",
        confirm: "Delete",
        abort: "Cancel",
    }).then(() => {
        activeRaid.value.players = activeRaid.value.players.filter(p => p.id != player.id);
        if (activePlayerId.value == player.id) {
            if (activeRaid.value.players.length)
                activePlayerId.value = activeRaid.value.players[0].id;
            else
                activePlayerId.value = null;
        }
    });
};
const playerCopyOptions = computed(() => {
    if (!activeRaid.value)
        return [];
    let options = [];
    for (let player of activeRaid.value.players) {
        if (player.id == playerModel.value.id)
            continue;
        options.push({value: player.id, title: player.name});
    }
    return options;
});
const createPlayerOpen = () => {
    if (!activeRaid.value)
        return;
    playerModel.value = defaultPlayer();
    playerModel.value.name = "";
    playerModel.value.race = activeRaid.value.faction == "Alliance" ? "Gnome" : "Undead";
    playerEdit.value.open(true);
};
const editPlayerOpen = (player) => {
    playerModel.value = _.cloneDeep(player);
    playerEdit.value.open(true);
};
const updatePlayer = () => {
    let isImport = playerImport.value.isOpen;
    playerEdit.value.close();
    playerImport.value.close();
    if (!activeRaid.value)
        return;

    let player = null;
    if (isImport) {
        // Overwrite player
        if (playerModelCopy.value) {
            let copy = activeRaid.value.players.find(p => p.id == playerModelCopy.value);
            if (copy) {
                if (playerImportConfig.value) {
                    player = _.cloneDeep(playerModel.value);
                    player.id = copy.id;
                    player.name = copy.name;
                    if (!playerImportLoadout.value)
                        player.loadout = _.cloneDeep(copy.loadout);
                }
                else {
                    player = _.cloneDeep(copy);
                    if (playerImportLoadout.value)
                        player.loadout = _.cloneDeep(playerModel.value.loadout);
                }
            }
        }
    }
    else {
        if (playerModelCopy.value) {
            let copy = activeRaid.value.players.find(p => p.id == playerModelCopy.value);
            if (copy) {
                player = _.cloneDeep(copy);
                player.id = playerModel.value.id;
                player.name = playerModel.value.name;
                player.race = playerModel.value.race;
            }
            playerModelCopy.value = null;
        }
    }

    if (!player)
        player = _.cloneDeep(playerModel.value);

    let index = activeRaid.value.players.findIndex(r => r.id == player.id);
    if (index != -1)
        activeRaid.value.players[index] = player;
    else
        activeRaid.value.players.push(player);

    syncBuffs();

    activePlayerId.value = player.id;
};
const specFromTalents = (talents) => {
    let count = [0, 0, 0];
    for (let i = 0; i < talents.length; i++) {
        if (talents[i] > 0)
            count[Math.floor(i / 16)]++;
    }

    let max = 0, tree = 0;
    for (let i = 0; i < count.length; i++) {
        if (count[i] > max) {
            max = count[i];
            tree = i;
        }
    }

    let trees = ["arcane", "fire", "frost"];

    return trees[tree];
};
const playerSpecIcon = (player) => {
    return "spec_"+specFromTalents(player.talents);
};
const raceOptions = computed(() => {
    if (activeRaid.value && activeRaid.value.faction == "Alliance") {
        return [
            { value: "Gnome", title: "Gnome" },
            { value: "Human", title: "Human" },
        ];
    }

    return [
        { value: "Troll", title: "Troll" },
        { value: "Undead", title: "Undead" },
    ];
});
const talentPreset = ref(null);
const talentPresetOptions = computed(() => {
    return talentPresets().map(t => { return {title: t.name, value: t.talents}; });
});
const setTalentPreset = () => {
    if (!activePlayer.value || !talentPreset.value)
        return;
    activePlayer.value.talents = talentPreset.value;
    nextTick(() => { talentPreset.value = null; });
};

/*
 * Main panel UI
 */
const activeTab = ref("config");
const activeSlot = ref("head");
const activeGearType = ref("gear");
const activeResultTab = ref("overview");

/*
 * Config UI
 */
const otherPlayerOptions = computed(() => {
    if (!activeRaid.value)
        return {};
    let options = [];
    for (let player of activeRaid.value.players) {
        if (player.id == activePlayerId.value)
            continue;
        options.push({value: player.id, title: player.name});
    }
    return options;
});
const playerConfigExclusive = (e, key, others) => {
    if (!_.isArray(others))
        others = [others];

    if (e.target.checked) {
        for (let other of others) {
            if (other == key)
                continue;
            activePlayer.value[other] = false;
        }
    }
};
const playerRadioToggle = (val, key) => {
    if (val == activePlayer.value[key])
        activePlayer.value[key] = 0;
};
const talentImport = ref("");
const importTalents = () => {
    let talents = parseTalents(talentImport.value);
    if (talents)
        activePlayer.value.talents = talents;
    else
        alert("Could not parse talent URL");
    talentImport.value = "";
};

const onSyncBuffs = () => {
    syncBuffs();
};
const syncBuffs = () => {
    if (!activeRaid.value._sync_buffs)
        return;
    let skip = [
        "id", "name", "race", "stats", "level", "apl",
        "talents", "loadout", "bonus_stats",
    ];
    for (let player of activeRaid.value.players) {
        if (player.id == activePlayer.value.id)
            continue;
        for (let key in activePlayer.value) {
            if (skip.includes(key))
                continue;
            player[key] = activePlayer.value[key];
        }
    }
};

/*
 * Item UI
 */
const paperdollSlots = (pos) => {
    if (pos == "left") {
        return [
            "head", "neck", "shoulder",
            "back", "chest", "wrist", "hands",
        ];
    }
    if (pos == "right") {
        return [
            "waist", "legs", "feet",
            "finger1", "finger2", "trinket1", "trinket2",
        ];
    }
    if (pos == "bottom") {
        return [
            "main_hand", "off_hand", "ranged",
        ];
    }
};
const paperdollClick = (slot, type) => {
    activeSlot.value = slot;
    activeGearType.value = type ? type : "gear";
    nextTick(() => {
        if (itemSearchInput.value)
            itemSearchInput.value.focus();
    });
};
const itemSearch = ref("");
const itemSearchInput = ref();
const itemSorting = ref({
    name: "ilvl",
    order: "desc",
});
const itemSort = (items, sorting) => {
    if (!sorting || !sorting.name)
        return items;

    let type = null;
    for (let i=0; i<items.length; i++) {
        let value = _.get(items[i], sorting.name, null);
        if (value !== null) {
            type = typeof(value);
            if (type == "object") {
                if (_.isArray(value))
                    type = "array";
                else
                    continue;
            }
            break;
        }
    }

    if (type === null)
        return items;

    return items.sort((a, b) => {
        let av = _.get(a, sorting.name, null);
        let bv = _.get(b, sorting.name, null);
        let result = 0;

        if (sorting.name == "phase") {
            if (!av) av = 1;
            if (!bv) bv = 1;
        }

        if (type == "string") {
            try { av = av.toString(); } catch(e) { av = ""; };
            try { bv = bv.toString(); } catch(e) { bv = ""; };
            result = av.localeCompare(bv);
        }
        else if (type == "number") {
            av = parseFloat(av);
            bv = parseFloat(bv);
            if (isNaN(av)) av = 0;
            if (isNaN(bv)) bv = 0;
            result = av - bv;
        }
        else if (type == "array") {
            av = _.get(av, "length", 0);
            bv = _.get(bv, "length", 0);
            result = av - bv;
        }

        if (sorting.order == "desc" && result != 0)
            result = result < 0 ? 1 : -1;

        // if (result == 0)
        //     result = a.title.localeCompare(b.title);

        return result;
    });
};
const itemList = computed(() => {
    let data = {
        type: activeGearType.value,
        slot: loadoutSlotToItemSlot(activeSlot.value),
        list: [],
    };

    if (data.type == "enchant")
        data.list = _.clone(items.enchants[data.slot]);
    else
        data.list = _.clone(items.gear[data.slot]);

    data.list = data.list.filter(item => {
        if (itemSearch.value.length) {
            if (item.title.toLowerCase().indexOf(itemSearch.value.toLowerCase()) == -1)
                return false;
        }
        if (item.hasOwnProperty("faction") && item.faction != activeRaid.value.faction)
            return false;
        return true;
    });

    data.list = itemSort(data.list, itemSorting.value);

    return data;
});
const itemSearchEnter = () => {
    if (itemList.value.list.length)
        itemClick(itemList.value.list[0]);
};
const itemSearchUp = () => {
    if (!itemList.value.list.length || !activePlayer.value || !activeSlot.value)
        return;
    let current = activePlayer.value.loadout[activeSlot.value].item_id;
    if (!current)
        return;
    let index = itemList.value.list.findIndex(i => i.id == current);
    activePlayer.value.loadout[activeSlot.value].item_id = itemList.value.list[(index+itemList.value.list.length-1)%itemList.value.list.length].id;
};
const itemSearchDown = () => {
    if (!itemList.value.list.length || !activePlayer.value || !activeSlot.value)
        return;
    let current = activePlayer.value.loadout[activeSlot.value].item_id;
    if (!current)
        return itemSearchEnter();
    let index = itemList.value.list.findIndex(i => i.id == current);
    activePlayer.value.loadout[activeSlot.value].item_id = itemList.value.list[(index+1)%itemList.value.list.length].id;
};
const itemClick = (item) => {
    if (!activePlayer.value || !activeSlot.value)
        return;

    let loadout = activePlayer.value.loadout[activeSlot.value];
    let key = activeGearType.value == "enchant" ? "enchant_id" : "item_id";
    if (loadout[key] == item.id) {
        loadout[key] = null;
    }
    else {
        if (key == "item_id" && activeSlot.value == "off_hand") {
            let mh = getItem("main_hand", activePlayer.value.loadout["main_hand"].item_id);
            if (mh.twohand)
                return;
        }
        else if (key == "item_id" && activeSlot.value == "main_hand" && item.twohand) {
            activePlayer.value.loadout["off_hand"].item_id = null;
        }

        if (item.unique) {
            let other = otherSlot(activeSlot.value);
            if (other && activePlayer.value.loadout[other].item_id) {
                if (item.unique === true) {
                    if (activePlayer.value.loadout[other].item_id == item.id)
                        return;
                }
                // Unique category
                else {
                    let otherItem = getItem(other, activePlayer.value.loadout[other].item_id);
                    if (otherItem && otherItem.unique && otherItem.unique === item.unique)
                        return;
                }
            }
        }

        loadout[key] = item.id;
    }

    refreshTooltips();
};
const copyLoadoutPlayer = ref(null);
const copyLoadout = (playerId) => {
    if (!activePlayer.value)
        return;
    let player = activeRaid.value.players.find(p => p.id == playerId);
    if (player && player.id != activePlayerId.value) {
        activePlayer.value.loadout = _.cloneDeep(player.loadout);
        refreshTooltips();
    }
    nextTick(() => { copyLoadoutPlayer.value = null });
};

const refreshTooltips = () => {
    if (window.$WowheadPower) {
        window.$WowheadPower.refreshLinks();
        nextTick(window.$WowheadPower.refreshLinks);
    }
};

/*
 * APL UI
 */
const loadApls = () => {
    let apls = window.localStorage.getItem("apls");
    if (apls)
        apls = JSON.parse(apls);
    if (_.isEmpty(apls))
        apls = [];
    return apls;
};
const saveApls = (data) => {
    data = data.filter(a => !isDefaultApl(a.id));
    window.localStorage.setItem("apls", JSON.stringify(data));
};
const selectApl = (data) => {
    if (!activePlayer.value)
        return;
    activePlayer.value.apl = data;
};
const apls = ref(loadApls());
const editApl = ref();
const aplModel = ref(null);
const aplTargetOptions = computed(() => {
    return apls.value.map(a => { return {value: a.id, title: a.name}; });
});
const copyPlayerApl = () => {
    if (!playerModelCopy.value || !activePlayer.value)
        return;
    let player = activeRaid.value.players.find(p => p.id == playerModelCopy.value);
    if (!player)
        return;
    activePlayer.value.apl = _.cloneDeep(player.apl);
    nextTick(() => { playerModelCopy.value = null; });
};
const editAplOpen = () => {
    aplModel.value = _.cloneDeep(activePlayer.value.apl);
    if (isDefaultApl(aplModel.value.id)) {
        if (aplModel.value.name == "Blank")
            aplModel.value.name = "";
        else
            aplModel.value.name+= " copy";
        aplModel.value.id = common.uuid();
    }
    if (editApl.value)
        editApl.value.open(true);
};
const deleteApl = (id) => {
    if (isDefaultApl(id))
        return;
    apls.value = apls.value.filter(a => a.id != id);
    saveApls(apls.value);
};
const updateApl = () => {
    if (!aplModel.value)
        return;

    let data = _.cloneDeep(aplModel.value);
    let index = _.findIndex(apls.value, {id: data.id});
    if (index == -1)
        apls.value.push(data);
    else
        apls.value.splice(index, 1, data);

    if (activePlayer.value && activePlayer.value.apl.id == data.id)
        activePlayer.value.apl.name = data.name;

    saveApls(apls.value);
    if (editApl.value)
        editApl.value.close();
};

/*
 * Export keys
 * We use these to make exports smaller by removing the keys and storing the values as arrays
 *
 * DO NOT CHANGE ORDER, REMOVE KEYS, OR ADD KEYS IN THE MIDDLE OF THE ARRAYS
 * ONLY ADD KEYS TO THE END
 * OTHERWISE IMPORTS WILL BREAK
 */
const raidExportKeys = () => {
    return [
        "name", "faction", "config", "players", "_sync_buffs",
    ];
};
const configExportKeys = () => {
    return [
        "rng_seed", "duration", "duration_variance", "avg_spell_dmg",
        "target_level", "target_resistance", "targets", "distance",
        "reaction_time", "player_delay",
        "curse_of_elements", "curse_of_shadows", "judgement_of_wisdom",
    ];
};
const playerExportKeys = () => {
    return [
        "name", "race", "level", "apl",
        "mage_armor", "mana_spring", "imp_mana_spring",
        "dmf_dmg", "soul_revival", "traces_of_silithyst",
        "arcane_intellect", "divine_spirit", "motw", "imp_motw", "moonkin_aura",
        "blessing_of_wisdom", "imp_blessing_of_wisdom", "blessing_of_kings",
        "atiesh_mage", "atiesh_warlock", "infallible_mind",
        "songflower", "rallying_cry", "warchiefs_blessing", "spirit_of_zandalar",
        "elixir_firepower", "elixir_greater_firepower", "elixir_frost_power",
        "elixir_arcane", "elixir_greater_arcane",
        "food", "flask", "weapon_oil",
        "bonus_stats", "talents", "loadout",
    ];
};
const aplExportKeys = () => {
    return ["type", "version", "items"];
};
const aplItemExportKeys = () => {
    return ["condition", "action", "status"];
};
const aplConditionExportKeys = () => {
    return ["condition_type", "op", "conditions", "values"];
};
const aplActionExportKeys = () => {
    // Add target id if we add interface for it
    // return ["key", "sequence", "target_id"];
    return ["key", "sequence"];
};
const aplValueExportKeys = () => {
    return ["value_type", "vstr", "vfloat", "vint"];
};
const exportSerialize = (keys, data, strict) => {
    if (!data.hasOwnProperty("x"))
        data.x = [];
    for (let key of keys) {
        let value = _.get(data, key, 0);
        if (value === true)
            value = 1;
        else if (value === false)
            value = 0;
        data.x.push(value);
        delete data[key];
    }
    if (strict) {
        for (let key in data) {
            if (key != "x")
                delete data[key];
        }
    }
};
const importDeserialize = (keys, data, ref) => {
    const objValue = (k, v) => {
        if (ref.hasOwnProperty(k)) {
            if (v === undefined)
                return ref[k];
            if (ref[k] === false || ref[k] === true)
                return v === 1 || v === true;
        }
        return v;
    };

    if (data.hasOwnProperty("x")) {
        for (let i in data.x) {
            let k = keys[i];
            data[k] = objValue(k, data.x[i]);
        }
        delete data.x;
    }

    for (let key in ref) {
        if (!data.hasOwnProperty(key))
            data[key] = ref[key];
    }

    return data;
};

/*
 * Export UI
 */
const importMessage = ref(null);
const exportType = ref("raid");
const exportTypeOptions = computed(() => {
    let options = [{value: "raid", title: "Raid"}];
    if (activePlayer.value)
        options.push({value: "player", title: "Player"});
    return options;
});
const statsExportData = (stats) => {
    let data = [];
    let keys = _.keys(common.stats());
    for (let key of keys)
        data.push(_.get(stats, key, 0));
    return data;
};
const statsImportData = (data) => {
    let stats = common.stats();
    let keys = _.keys(common.stats());
    for (let i of data)
        stats[keys[i]] = data[i];
    return stats;
};
const loadoutExportData = (loadout) => {
    loadout = _.cloneDeep(loadout);
    for (let key in loadout) {
        if (loadout[key].item_id)
            loadout[key] = [loadout[key].item_id, loadout[key].enchant_id ? loadout[key].enchant_id : 0];
        else
            loadout[key] = [0,0];
    }
    let data = [];
    for (let key of loadoutSlots())
        data.push(loadout[key]);
    return data;
};
const loadoutImportData = (data) => {
    let loadout = baseLoadout();
    let slots = loadoutSlots();
    for (let i in slots) {
        loadout[slots[i]] = {
            item_id: (data[i][0] ? data[i][0] : null),
            enchant_id: (data[i][1] ? data[i][1] : null),
        };
    }
    return loadout;
};
const talentExportData = (talents) => {
    let str = "";
    for (let i in talents) {
        if (i%16 == 0 && i != 0 && i < 33)
            str+= "-";
        str+= talents[i];
    }
    str = str.replace(/[0]+-/, "-");
    str = str.replace(/[0]+$/, "");
    str = str.replace(/[-]+$/, "");
    return str;
};
const talentImportData = (talents) => {
    return parseWowheadTalents(talents);
};
const aplExportData = (data) => {
    let minimize = (obj) => {
        delete obj.id;
        delete obj.title;
        if (obj.condition) {
            minimize(obj.condition);
            exportSerialize(aplConditionExportKeys(), obj.condition);
        }
        if (obj.action) {
            minimize(obj.action);
            exportSerialize(aplActionExportKeys(), obj.action, true);
        }
        if (obj.items)
            obj.items.forEach((o) => { minimize(o); exportSerialize(aplItemExportKeys(), o); });
        if (obj.conditions)
            obj.conditions.forEach((o) => { minimize(o); exportSerialize(aplConditionExportKeys(), o); });
        if (obj.actions)
            obj.actions.forEach((o) => { minimize(o); exportSerialize(aplActionExportKeys(), o, true); });
        if (obj.sequence)
            obj.sequence.forEach((o) => { minimize(o); exportSerialize(aplActionExportKeys(), o, true); });
        if (obj.values)
            obj.values.forEach((o) => { minimize(o); exportSerialize(aplValueExportKeys(), o); });
    };

    if (isDefaultApl(data.id)) {
        return {id: data.id};
    }
    else {
        data = _.cloneDeep(data);
        delete data.name;
        minimize(data);
        exportSerialize(aplExportKeys(), data);
        return data;
    }
};
const aplImportData = (data) => {
    let restore = (obj) => {
        if (!obj.id)
            obj.id = common.uuid();
        if (obj.condition) {
            importDeserialize(aplConditionExportKeys(), obj.condition, aplData.condition());
            restore(obj.condition);
        }
        if (obj.action) {
            importDeserialize(aplActionExportKeys(), obj.action, aplData.action());
            restore(obj.action);
        }
        if (obj.items)
            obj.items.forEach((o) => { importDeserialize(aplItemExportKeys(), o, aplData.item()); restore(o); });
        if (obj.conditions)
            obj.conditions.forEach((o) => { importDeserialize(aplConditionExportKeys(), o, aplData.condition()); restore(o); });
        if (obj.actions)
            obj.actions.forEach((o) => { importDeserialize(aplActionExportKeys(), o, aplData.getAction(o[0])); restore(o); });
        if (obj.sequence)
            obj.sequence.forEach((o) => { importDeserialize(aplActionExportKeys(), o, aplData.getAction(o[0])); restore(o); });
        if (obj.values)
            obj.values.forEach((o) => { importDeserialize(aplValueExportKeys(), o, aplData.value()); restore(o); });
    };

    data = _.cloneDeep(data);

    if (data.id) {
        let ap = defaultApls().find(a => a.id == data.id);
        if (ap)
            return ap;
    }

    importDeserialize(aplExportKeys(), data, aplData.apl());
    restore(data);

    return data;
};
const exportPlayerData = (player) => {
    player = _.cloneDeep(player);
    player.bonus_stats = statsExportData(player.bonus_stats);
    player.talents = talentExportData(player.talents);
    player.loadout = loadoutExportData(player.loadout);
    player.apl = aplExportData(player.apl);
    delete player.id;
    delete player.stats;
    delete player.items;
    exportSerialize(playerExportKeys(), player);
    return player;
};
const importPlayerData = (data) => {
    data = _.cloneDeep(data);
    importDeserialize(playerExportKeys(), data, defaultPlayer());
    data.loadout = loadoutImportData(data.loadout);
    data.bonus_stats = statsImportData(data.bonus_stats);
    data.talents = talentImportData(data.talents);
    data.apl = aplImportData(data.apl);
    return data;
};
const exportRaidData = (raid) => {
    raid = _.cloneDeep(raid);
    for (let p in raid.players)
        raid.players[p] = exportPlayerData(raid.players[p]);
    exportSerialize(configExportKeys(), raid.config);
    exportSerialize(raidExportKeys(), raid);
    delete raid.id;
    return raid;
};
const importRaidData = (data) => {
    data = _.cloneDeep(data);
    importDeserialize(raidExportKeys(), data, defaultRaid());
    data.config = importDeserialize(configExportKeys(), data.config, defaultConfig());

    for (let p in data.players)
        data.players[p] = importPlayerData(data.players[p]);

    return data;
};
const exportSuccess = ref(false);
const exportSubmit = () => {
    exportSuccess.value = false;

    let data = {
        exp: exportType.value,
        v: "1.0",
        data: null,
    };

    if (exportType.value == "raid") {
        data.data = exportRaidData(activeRaid.value);
    }
    else if (exportType.value == "player" && activePlayer.value) {
        data.data = exportPlayerData(activePlayer.value);
    }
    else {
        alert("Invalid export type");
        return;
    }

    data = compressToEncodedURIComponent(JSON.stringify(data));
    data = window.location.origin+"#mse="+data;
    copyToClipboard(data);
    nextTick(() => { exportSuccess.value = true; });
};
const importData = ref("");
const importSubmit = () => {
    if (importString(importData.value)) {
        importData.value = "";
    }
};
const importString = (str) => {
    if (!str.length)
        return;

    let type = null;
    let data = null;
    try {
        data = JSON.parse(str);
        if (data.phase)
            type = "60up";
        else if (data.gear && data.gear.items)
            type = "wse";
    }
    catch (e) {
        let m = str.match(/https\:\/\/(vanilla|sod)\.warcraftlogs\.com\/reports\/([a-z0-9]+)/i);
        if (m) {
            type = "wcl";
            data = m[2];
        }
        else {
            type = "native";
            data = str;
        }
    }

    // TODO: External import sources
    try {
        if (type == "60up") {
            importSixtyUpgrades(data);
            return true;
        }
        else if (type == "wse") {
            importWSE(data);
            return true;
        }
        else if (type == "wcl") {

        }
        else if (type == "native") {
            importNativeString(str);
            return true;
        }
    }
    catch(e) {
        console.error(e);
    }

    // No matching imports
    alert("Unrecognized format");
    return false;
};
const importNativeString = (str) => {
    let index = str.indexOf("#mse=");
    if (index != -1)
        str = str.substr(index+5);
    let data = JSON.parse(decompressFromEncodedURIComponent(str));
    importNative(data);
};
const importNative = (data) => {
    if (!data.exp)
        throw "Invalid export type";
    if (data.exp == "raid") {
        let raid = importRaidData(data.data);
        raidModel.value = raid;
        raidEdit.value.open(true);
    }
    else if (data.exp == "player") {
        let player = importPlayerData(data.data);
        playerModel.value = player;
        playerImport.value.open(true);
    }
    else {
        throw "Invalid export type";
    }
};
const importSixtyUpgrades = (data) => {
    if (!data.items)
        throw("No items found");

    let player = defaultPlayer();
    player.name = _.get(data, "character.name");
    player.race = _.capitalize(_.get(data, "character.race", "Undead"));

    let convertSlot = (slot) => {
        slot = slot.toLowerCase();
        slot = slot.replace("finger_", "finger");
        slot = slot.replace("trinket_", "trinket");
        slot = slot.replace("shoulders", "shoulder");
        slot = slot.replace("wrists", "wrist");
        return player.loadout.hasOwnProperty(slot) ? slot : null;
    };

    let errors = [];

    for (let _item of data.items) {
        let slot = convertSlot(_item.slot);
        let id = _item.id;
        if (_item.hasOwnProperty("suffixId"))
            id+= ":"+_item.suffixId;
        let item = getItem(slot, id);
        if (!item)
            item = items.gear[loadoutSlotToItemSlot(slot)].find(i => i.title == _item.name);
        if (!item) {
            player.loadout[slot].item_id = null;
            errors.push("Could not find item: "+_item.name);
            continue;
        }

        let enchant = null;
        if (_item.enchant) {
            if (_item.enchant.id)
                enchant = items.enchants[slot].find(e => e.enchantment_id == _item.enchant.id);
            if (!enchant && _item.enchant.spellId)
                enchant = items.enchants[slot].find(e => e.id == _item.enchant.spellId);
            if (!enchant)
                errors.push("Could not find enchant: "+_item.enchant.name);
        }

        player.loadout[slot].item_id = item.id;
        player.loadout[slot].enchant_id = enchant ? enchant.id : null;
    }

    if (data.talents && data.talents.length) {
        let flatTree = talentTree.trees.reduce((acc, t) => [...acc, ...t.talents.rows.flat()], []);
        player.talents = baseTalents();
        for (let _talent of data.talents) {
            let index = flatTree.findIndex(t => t.spellIds.indexOf(_talent.spellId) != -1);
            if (index == -1)
                errors.push("Could not find talent: "+_talent.name);
            else
                player.talents[index] = _talent.rank;
        }
    }

    playerModel.value = player;
    importMessage.value = errors.join("<br>");
    playerImport.value.open(true);
};
const importWSE = (data) => {
    if (!data.gear || !data.gear.items)
        throw("No items found");

    let player = defaultPlayer();
    let slots = loadoutSlots();
    let errors = [];

    for (let i in data.gear.items) {
        let _item = data.gear.items[i];
        if (!_item)
            continue;
        let slot = slots[i];
        let item = getItem(slot, _item.id);
        if (!item) {
            player.loadout[slot].item_id = null;
            errors.push("Could not find item: "+_item.id);
            continue;
        }

        let enchant = null;
        if (_item.enchant) {
            if (_item.enchant.id)
                enchant = items.enchants[slot].find(e => e.enchantment_id == _item.enchant);
            if (!enchant)
                errors.push("Could not find enchant: "+_item.enchant);
        }

        player.loadout[slot].item_id = item.id;
        player.loadout[slot].enchant_id = enchant ? enchant.id : null;
    }

    if (data.race)
        player.race = _.capitalize(data.race);
    if (raceFaction(player.race) != activeRaid.value.faction)
        player.race = convertRace(player.race);

    if (data.talents)
        player.talents = parseWowheadTalents(data.talents);

    playerModel.value = player;
    importMessage.value = errors.join("<br>");
    playerImport.value.open(true);
};

/*
 * Result UI
 */
const resultHidden = ref(false);
const resultOpen = ref(false);
const closeResult = () => {
    resultOpen.value = false;
};
const openResult = () => {
    resultOpen.value = true;
};
const playerDps = (player) => {
    if (result.value && settings.normalize_ignite) {
        return player.dps - player.ignite_dps + result.value.ignite_dps / result.value.players.length;
    }

    return player.dps;
};
const filterHistogram = ref(null);
const filterHistogramOptions = computed(() => {
    let options = [{value: "total", title: "Total dps"}];
    if (result.value.ignite_dps)
        options.push({value: "ignite", title: "Ignite"});
    return options;
});
const histogramData = computed(() => {
    if (filterHistogram.value == "ignite")
        return result.value.ignite_histogram;
    return result.value.histogram;
});

/*
 * Watchers
 */
watch(settings, saveSettings, {deep : true});
watch(raids, saveRaids, {deep : true});
watch(() => settings.raid_id, (value) => {
    let raid = raids.value.find(raid => raid.id == value);
    if (raid && raid.players.length) {
        activePlayerId.value = raid.players[0].id;
    }
    else {
        activePlayerId.value = null;
        activeTab.value = "config";
    }
});
watch(() => itemSearch.value, refreshTooltips);
watch(() => activeTab.value, () => {
    exportSuccess.value = false;
    refreshTooltips();
});
watch(() => activeGearType.value, refreshTooltips);
watch(() => activeSlot.value, refreshTooltips);
watch(() => activePlayer.value, () => {
    if (activePlayer.value) {
        playerStats.value = visualStats(activePlayer.value);
        syncBuffs();
        refreshTooltips();
    }
}, {deep: true});
watch(() => result.value, () => {
    activeResultTab.value = "overview";
    resultHidden.value = false;
});

/*
 * Events
 */
onMounted(() => {
    if (activeRaid.value && activeRaid.value.players.length) {
        activePlayerId.value = activeRaid.value.players[0].id;
        activeTab.value = "loadout";
        nextTick(() => {
            playerStats.value = visualStats(activePlayer.value);
        });
    }

    if (window.location.hash) {
        let hash = window.location.hash.substr(1);
        if (hash.substr(0, 4) == "mse=") {
            try {
                importNativeString(hash.substr(4));
            }
            catch(e) {
                alert("Could not import data");
                console.log(e);
            }
            window.location.hash = "";
        }
    }
});
</script>

<template>
    <div class="app">
        <div id="main">
            <div class="left">
                <div class="raid">
                    <div class="current-raid" @click="raidSelectOpen = !raidSelectOpen">
                        <template v-if="activeRaid">
                            <wowicon class="faction" :icon="activeRaid.faction" />
                            <div class="name">{{ activeRaid.name }}</div>
                            <micon class="caret" icon="keyboard_arrow_down" />
                        </template>
                        <template v-else>
                            <div class="name"><i>No raid</i></div>
                        </template>
                    </div>
                    <div class="raid-select" v-if="raidSelectOpen">
                        <div class="raids">
                            <div
                                class="raid"
                                :class="{active: activeRaid.value == raid.id}"
                                v-for="raid in raids"
                                :key="raid.id"
                                @click="selectRaid(raid.id)"
                            >
                                <wowicon class="faction" :icon="raid.faction" />
                                <span class="middle info">
                                    <div class="name">{{ raid.name }}</div>
                                    <div class="players">{{ raid.players.length }} players</div>
                                </span>
                                <micon class="delete" icon="delete" @click.stop="confirmDeleteRaid(raid)" />
                            </div>
                        </div>
                        <button class="create" @click="createRaidOpen">
                            <micon icon="add" />
                            <span class="middle">Create raid</span>
                        </button>
                    </div>
                </div>

                <div class="team" v-if="activeRaid">
                    <div class="players">
                        <div
                            class="player"
                            :class="{active: activePlayerId == player.id}"
                            v-for="player in activeRaid.players"
                            :key="player.id"
                            @click="selectPlayer(player.id)"
                        >
                            <wowicon class="race" :icon="player.race" />
                            <wowicon class="spec" :icon="playerSpecIcon(player)" />
                            <div class="middle name">{{ player.name }}</div>
                            <micon class="delete" icon="delete" @click.stop="confirmDeletePlayer(player)" />
                        </div>
                    </div>
                    <button class="create" @click="createPlayerOpen">
                        <micon icon="add" />
                        <span class="middle">Add player</span>
                    </button>
                </div>

                <div class="sim">
                    <template v-if="isRunning">
                        <div class="progress">
                            <div class="circle middle">
                                <progress-circle :value="simProgress.progress" />
                                <div class="center">{{ (simProgress.progress * 100).toFixed() }}%</div>
                            </div>
                            <div class="dps middle">
                                <div class="title">DPS</div>
                                <div class="value">{{ simProgress.dps.toFixed(1) }}</div>
                            </div>
                        </div>
                    </template>
                    <template v-else>
                        <div class="run">
                            <div><button class="btn btn-primary block large" @click="runMultiple">Run</button></div>
                            <div><button class="btn btn-text" @click="runSingle">Single iteration</button></div>
                        </div>
                        <div class="result" v-if="result && !resultHidden">
                            <div class="close" @click="resultHidden = true">
                                <micon icon="close" />
                            </div>
                            <div class="dps">
                                <span class="value">{{ playerDps(result.players[0]).toFixed(1) }}</span>
                                <span class="title">{{ result.players[0].name }}</span>
                            </div>
                            <div class="dps">
                                <span class="value">{{ result.dps.toFixed(1) }}</span>
                                <span class="title">Total</span>
                            </div>
                            <button class="link" @click.stop="openResult">
                                <span class="middle">See result</span>
                                <micon icon="keyboard_double_arrow_right" />
                            </button>
                        </div>
                    </template>
                </div>
            </div>

            <div class="right" v-if="activeRaid">
                <div class="tabs">
                    <button class="tab" :class="{active: activeTab == 'config'}" @click="activeTab = 'config'">
                        Config
                    </button>
                    <template v-if="activePlayer">
                        <button class="tab" :class="{active: activeTab == 'loadout'}" @click="activeTab = 'loadout'">
                            Gear
                        </button>
                        <button class="tab" :class="{active: activeTab == 'talents'}" @click="activeTab = 'talents'">
                            Talents
                        </button>
                        <button class="tab" :class="{active: activeTab == 'rotation'}" @click="activeTab = 'rotation'">
                            Rotation
                        </button>
                    </template>
                    <button class="tab" :class="{active: activeTab == 'export'}" @click="activeTab = 'export'">
                        Export
                    </button>
                    <button class="tab" :class="{active: activeTab == 'import'}" @click="activeTab = 'import'">
                        Import
                    </button>
                    <div class="github">
                        <a href="https://github.com/Cheesehyvel/magesim-vanilla" target="_blank">
                            <img src="/img/github-mark.svg" alt="Github">
                        </a>
                    </div>
                </div>

                <div class="config" v-if="activeTab == 'config'">
                    <div class="form-box config-raid">
                        <div class="title">Raid config</div>
                        <div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>Name</label>
                                    <input type="text" v-model.number="activeRaid.name">
                                </div>
                                <div class="form-item">
                                    <label>Faction</label>
                                    <select-simple v-model="activeRaid.faction" :options="factionOptions" />
                                </div>
                            </div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>Fight duration</label>
                                    <input type="text" v-model.number="activeRaid.config.duration">
                                </div>
                                <div class="form-item">
                                    <label>Variance (+/-)</label>
                                    <input type="text" v-model.number="activeRaid.config.duration_variance">
                                </div>
                            </div>
                            <div class="form-item">
                                <label>Debuffs</label>
                                <div class="icon-checkboxes">
                                    <label>
                                        <input type="checkbox" v-model="activeRaid.config.curse_of_elements">
                                        <wowicon icon="curse_of_elements" />
                                        <tooltip>Curse of the Elements</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activeRaid.config.curse_of_shadows">
                                        <wowicon icon="curse_of_shadows" />
                                        <tooltip>Curse of Shadows</tooltip>
                                    </label>
                                    <label v-if="activeRaid.faction == 'Alliance'">
                                        <input type="checkbox" v-model="activeRaid.config.judgement_of_wisdom">
                                        <wowicon icon="judgement_of_wisdom" />
                                        <tooltip>Judgement of Wisdom</tooltip>
                                    </label>
                                </div>
                            </div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>Target level</label>
                                    <input type="text" v-model.number="activeRaid.config.target_level">
                                </div>
                                <div class="form-item">
                                    <label>Target resistance</label>
                                    <input type="text" v-model.number="activeRaid.config.target_resistance">
                                </div>
                            </div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>
                                        <span class="middle">Distance from target</span>
                                        <help>This only affects travel time.<br>No range checks are made.</help>
                                    </label>
                                    <input type="text" v-model.number="activeRaid.config.distance">
                                </div>
                                <div class="form-item">
                                    <label>Number of targets</label>
                                    <input type="text" v-model.number="activeRaid.config.targets">
                                </div>
                            </div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>
                                        <span class="middle">Reaction time</span>
                                        <help>In seconds</help>
                                    </label>
                                    <input type="text" v-model.number="activeRaid.config.reaction_time">
                                </div>
                                <div class="form-item">
                                    <label>
                                        <span class="middle">Player staggering</span>
                                        <help>Seconds between the start of each player.</help>
                                    </label>
                                    <input type="text" v-model.number="activeRaid.config.player_delay">
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="form-box config-player" v-if="activePlayer">
                        <div class="title">Player config</div>
                        <div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>Name</label>
                                    <input type="text" v-model.number="activePlayer.name">
                                </div>
                                <div class="form-item">
                                    <label>Race</label>
                                    <select-simple v-model="activePlayer.race" :options="raceOptions" />
                                </div>
                            </div>
                            <div class="form-item">
                                <label>Buffs</label>
                                <div class="icon-checkboxes">
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.arcane_intellect">
                                        <wowicon icon="arcane_intellect" />
                                        <tooltip>Arcane Intellect</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.mage_armor">
                                        <wowicon icon="mage_armor" />
                                        <tooltip>Mage Armor</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.divine_spirit">
                                        <wowicon icon="divine_spirit" />
                                        <tooltip>Divine Spirit</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.motw">
                                        <wowicon icon="motw" />
                                        <tooltip>Mark of the Wild</tooltip>
                                    </label>
                                    <label v-if="activePlayer.motw">
                                        <input type="checkbox" v-model="activePlayer.imp_motw">
                                        <wowicon icon="imp_motw" />
                                        <tooltip>Improved Mark of the Wild</tooltip>
                                        <micon class="imp" icon="add" />
                                    </label>
                                    <template v-if="activeRaid.faction == 'Alliance'">
                                        <label>
                                            <input type="checkbox" v-model="activePlayer.blessing_of_kings">
                                            <wowicon icon="blessing_of_kings" />
                                            <tooltip>Blessing of Kings</tooltip>
                                        </label>
                                        <label>
                                            <input type="checkbox" v-model="activePlayer.blessing_of_wisdom">
                                            <wowicon icon="blessing_of_wisdom" />
                                            <tooltip>Blessing of Wisdom</tooltip>
                                        </label>
                                    </template>
                                    <template v-else>
                                        <label>
                                            <input type="checkbox" v-model="activePlayer.mana_spring">
                                            <wowicon icon="mana_spring" />
                                            <tooltip>Mana Spring Totem</tooltip>
                                        </label>
                                        <label v-if="activePlayer.mana_spring">
                                            <input type="checkbox" v-model="activePlayer.imp_mana_spring">
                                            <wowicon icon="mana_spring" />
                                            <tooltip>Restorative totems (Improved mana spring)</tooltip>
                                            <micon class="imp" icon="add" />
                                        </label>
                                    </template>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.moonkin_aura">
                                        <wowicon icon="moonkin_aura" />
                                        <tooltip>Moonkin aura</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.atiesh_mage">
                                        <wowicon icon="atiesh" />
                                        <wowicon class="addon" icon="mage" />
                                        <tooltip>Atiesh aura from a mage in your party.</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.atiesh_warlock">
                                        <wowicon icon="atiesh" />
                                        <wowicon class="addon" icon="warlock" />
                                        <tooltip>Atiesh aura from a warlock in your party</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.dmf_dmg">
                                        <wowicon icon="dmf" />
                                        <tooltip>Sayge's Dark Fortune of Damage</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.soul_revival">
                                        <wowicon icon="soul_revival" />
                                        <tooltip>Soul Revival</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.traces_of_silithyst">
                                        <wowicon icon="traces_of_silithyst" />
                                        <tooltip>Traces of Silithyst</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.songflower">
                                        <wowicon icon="songflower" />
                                        <tooltip>Songflower</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.rallying_cry">
                                        <wowicon icon="rallying_cry" />
                                        <tooltip>Rallying Cry of the Dragonslayer</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.warchiefs_blessing">
                                        <wowicon icon="warchiefs_blessing" />
                                        <tooltip>Warchief's Blessing</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.spirit_of_zandalar">
                                        <wowicon icon="spirit_of_zandalar" />
                                        <tooltip>Spirit of Zandalar</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.infallible_mind">
                                        <wowicon icon="infallible_mind" />
                                        <tooltip>Infallible Mind</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.elixir_greater_firepower" @click="playerConfigExclusive($event, 'elixir_greater_firepower', 'elixir_firepower')">
                                        <wowicon icon="elixir_greater_firepower" />
                                        <tooltip>Elixir of Greater Firepower</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.elixir_firepower" @click="playerConfigExclusive($event, 'elixir_firepower', 'elixir_greater_firepower')">
                                        <wowicon icon="elixir_firepower" />
                                        <tooltip>Elixir of Firepower</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.elixir_greater_arcane" @click="playerConfigExclusive($event, 'elixir_greater_arcane', 'elixir_arcane')">
                                        <wowicon icon="elixir_greater_arcane" />
                                        <tooltip>Greater Arcane Elixir</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.elixir_arcane" @click="playerConfigExclusive($event, 'elixir_arcane', 'elixir_greater_arcane')">
                                        <wowicon icon="elixir_arcane" />
                                        <tooltip>Arcane Elixir</tooltip>
                                    </label>
                                    <label>
                                        <input type="checkbox" v-model="activePlayer.elixir_frost_power">
                                        <wowicon icon="elixir_frost_power" />
                                        <tooltip>Elixir of Frost Power</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.flask" :value="common.flasks.SUPREME_POWER" @click="playerRadioToggle(activePlayer.flask, 'flask')">
                                        <wowicon icon="flask_supreme_power" />
                                        <tooltip>Flask of Supreme Power</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.flask" :value="common.flasks.DISTILLED_WISDOM" @click="playerRadioToggle(activePlayer.flask, 'flask')">
                                        <wowicon icon="flask_distilled_wisdom" />
                                        <tooltip>Flask of Distilled Wisdom</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.weapon_oil" :value="common.weapon_oils.BLESSED_WIZARD" @click="playerRadioToggle(activePlayer.weapon_oil, 'weapon_oil')">
                                        <wowicon icon="weapon_oil_blessed_wizard" />
                                        <tooltip>Blessed Wizard Oil</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.weapon_oil" :value="common.weapon_oils.BRILLIANT_WIZARD" @click="playerRadioToggle(activePlayer.weapon_oil, 'weapon_oil')">
                                        <wowicon icon="weapon_oil_brilliant_wizard" />
                                        <tooltip>Brilliant Wizard Oil</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.weapon_oil" :value="common.weapon_oils.WIZARD" @click="playerRadioToggle(activePlayer.weapon_oil, 'weapon_oil')">
                                        <wowicon icon="weapon_oil_wizard" />
                                        <tooltip>Wizard Oil</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.weapon_oil" :value="common.weapon_oils.BRILLIANT_MANA" @click="playerRadioToggle(activePlayer.weapon_oil, 'weapon_oil')">
                                        <wowicon icon="weapon_oil_brilliant_mana" />
                                        <tooltip>Brilliant Mana Oil</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.food" :value="common.foods.RUNN_TUM" @click="playerRadioToggle(activePlayer.food, 'food')">
                                        <wowicon icon="food_runn_tum" />
                                        <tooltip>Runn Tum Tuber Surprise</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.food" :value="common.foods.NIGHTFIN" @click="playerRadioToggle(activePlayer.food, 'food')">
                                        <wowicon icon="food_nightfin" />
                                        <tooltip>Nightfin Soup</tooltip>
                                    </label>
                                    <label>
                                        <input type="radio" v-model="activePlayer.food" :value="common.foods.WELL_FED" @click="playerRadioToggle(activePlayer.food, 'food')">
                                        <wowicon icon="food_well_fed" />
                                        <tooltip>Well Fed</tooltip>
                                    </label>
                                </div>
                            </div>
                            <div class="form-item">
                                <checkbox label="Sync buffs" tip="Sync buffs between all players">
                                    <input type="checkbox" v-model="activeRaid._sync_buffs" @change="onSyncBuffs">
                                </checkbox>
                            </div>
                            <div class="form-title">Bonus stats</div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>Spell power</label>
                                    <input type="text" v-model.number="activePlayer.bonus_stats.sp">
                                </div>
                                <div class="form-item">
                                    <label>Crit %</label>
                                    <input type="text" v-model.number="activePlayer.bonus_stats.crit">
                                </div>
                                <div class="form-item">
                                    <label>Hit %</label>
                                    <input type="text" v-model.number="activePlayer.bonus_stats.hit">
                                </div>
                            </div>
                            <div class="form-cols">
                                <div class="form-item">
                                    <label>Int</label>
                                    <input type="text" v-model.number="activePlayer.bonus_stats.int">
                                </div>
                                <div class="form-item">
                                    <label>Spi</label>
                                    <input type="text" v-model.number="activePlayer.bonus_stats.spi">
                                </div>
                                <div class="form-item">
                                    <label>Mp5</label>
                                    <input type="text" v-model.number="activePlayer.bonus_stats.mp5">
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="form-box small config-settings">
                        <div class="title">Sim config</div>
                        <div>
                            <div class="form-item">
                                <label>Iterations</label>
                                <input type="text" v-model.number="settings.iterations">
                            </div>
                            <div class="form-item">
                                <label>
                                    <span class="middle">Threads</span>
                                    <help>
                                        Number of parallel simulations to run,<br> using multiple CPU cores.<br><br>
                                        Detected cores: <b>{{ detectedCores() }}</b>
                                    </help>
                                </label>
                                <input type="text" v-model.number="settings.threads">
                            </div>
                        </div>
                    </div>
                </div>

                <div class="loadout" v-if="activeTab == 'loadout' && activePlayer">
                    <div class="overview">
                        <div class="copy">
                            <select-simple
                                v-model="copyLoadoutPlayer"
                                :options="otherPlayerOptions"
                                @input="copyLoadout"
                                placeholder="Copy gear from..."
                             />
                        </div>
                        <div class="paperdoll">
                            <div :class="pos" v-for="pos in ['left', 'right', 'bottom']">
                                <div class="paperslot" :class="css(slot)" v-for="slot in paperdollSlots(pos)">
                                    <div
                                        class="paperv paperitem"
                                        :class="{active: activeSlot == slot && activeGearType == 'gear'}"
                                        @click="paperdollClick(slot)"
                                    >
                                        <a
                                            v-if="activePlayer.loadout[slot].item_id"
                                            :href="gearUrl(activePlayer, slot)"
                                            data-wh-icon-size="large"
                                            data-whtticon="false"
                                            @click.prevent
                                        ></a>
                                    </div>
                                    <div class="papers">
                                        <div
                                            class="paperv paperenchant"
                                            :class="{active: activeSlot == slot && activeGearType == 'enchant'}"
                                            v-if="items.enchants.hasOwnProperty(loadoutSlotToItemSlot(slot))"
                                            @click="paperdollClick(slot, 'enchant')"
                                        >
                                            <a
                                                v-if="activePlayer.loadout[slot].enchant_id"
                                                :href="spellUrl(activePlayer.loadout[slot].enchant_id)"
                                                data-wh-icon-size="large"
                                                data-whtticon="false"
                                                @click.prevent
                                            ></a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="stats">
                            <table>
                                <tbody>
                                    <tr>
                                        <td>Spell power</td>
                                        <td>
                                            <span>
                                                {{ playerStats.sp }}
                                                <tooltip position="left"><spell-power :value="playerStats" /></tooltip>
                                            </span>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>Crit</td>
                                        <td>{{ playerStats.crit.toFixed(2) }}%</td>
                                    </tr>
                                    <tr>
                                        <td>Hit</td>
                                        <td>{{ playerStats.hit.toFixed() }}%</td>
                                    </tr>
                                    <tr>
                                        <td>Mana</td>
                                        <td>{{ playerStats.mana }}</td>
                                    </tr>
                                    <tr>
                                        <td>Intellect</td>
                                        <td>{{ playerStats.int }}</td>
                                    </tr>
                                    <tr>
                                        <td>Spirit</td>
                                        <td>{{ playerStats.spi }}</td>
                                    </tr>
                                    <tr>
                                        <td>Mp5</td>
                                        <td>{{ playerStats.mp5 }}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <div class="itemlist" v-if="activeSlot && activeGearType">
                        <div class="search">
                            <input
                                type="text"
                                class="search-q"
                                v-model="itemSearch"
                                ref="itemSearchInput"
                                placeholder="Search..."
                                @keydown.enter="itemSearchEnter"
                                @keydown.up="itemSearchUp"
                                @keydown.down="itemSearchDown"
                                @keydown.esc="itemSearch = ''"
                                autofocus
                            >
                            <div class="keyboard-help" @click="itemSearchInput.focus()">
                                <div class="title">
                                    <micon icon="keyboard" />
                                </div>
                                <div class="drop">
                                    <div><span>Esc:</span> <span>Clear search</span></div>
                                    <div><span>Enter:</span> <span>Equip first item in the list</span></div>
                                    <div><span>Up/Down:</span> <span>Equip previous/next item in the list</span></div>
                                </div>
                            </div>
                        </div>
                        <div class="items">
                            <table v-if="itemList.list">
                                <thead>
                                    <tr>
                                        <th class="title">
                                            <sort-link v-model="itemSorting" name="title">Name</sort-link>
                                        </th>
                                        <th v-if="itemList.type != 'enchant'">
                                            <sort-link v-model="itemSorting" name="ilvl" order="desc">ilvl</sort-link>
                                        </th>
                                        <th>
                                            <sort-link v-model="itemSorting" name="sp" order="desc">SP</sort-link>
                                        </th>
                                        <th>
                                            <sort-link v-model="itemSorting" name="crit" order="desc">Crit</sort-link>
                                        </th>
                                        <th>
                                            <sort-link v-model="itemSorting" name="hit" order="desc">Hit</sort-link>
                                        </th>
                                        <th>
                                            <sort-link v-model="itemSorting" name="int" order="desc">Int</sort-link>
                                        </th>
                                        <th>
                                            <sort-link v-model="itemSorting" name="spi" order="desc">Spi</sort-link>
                                        </th>
                                        <th>
                                            <sort-link v-model="itemSorting" name="mp5" order="desc">Mp5</sort-link>
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr
                                        :class="{active: itemList.type == 'enchant' ? activePlayer.loadout[activeSlot].enchant_id == item.id : activePlayer.loadout[activeSlot].item_id == item.id}"
                                        v-for="item in itemList.list"
                                        :key="item.id"
                                        @click="itemClick(item)"
                                    >
                                        <td class="title">
                                            <a
                                                :href="itemList.type == 'enchant' ? spellUrl(item.id) : itemUrl(item.id)"
                                                :class="'quality-'+_.get(item, 'q', itemList.type == 'enchant' ? 'uncommon' : 'epic')"
                                                data-whtticon="false"
                                                target="_blank"
                                                @click.prevent
                                            >
                                                {{ item.title }}
                                            </a>
                                        </td>
                                        <td v-if="itemList.type != 'enchant'">{{ item.ilvl }}</td>
                                        <td><spell-power :value="item" /></td>
                                        <td>{{ _.get(item, "crit", "") }}</td>
                                        <td>{{ _.get(item, "hit", "") }}</td>
                                        <td>{{ _.get(item, "int", "") }}</td>
                                        <td>{{ _.get(item, "spi", "") }}</td>
                                        <td>{{ _.get(item, "mp5", "") }}</td>
                                    </tr>
                                </tbody>
                            </table>
                            <div class="empty" v-else>
                                No results
                            </div>
                        </div>
                    </div>
                </div>

                <div class="talents" v-if="activeTab == 'talents' && activePlayer">
                    <div class="import">
                        <select-simple
                            v-model="talentPreset"
                            :options="talentPresetOptions"
                            placeholder="Select preset..."
                            @input="setTalentPreset"
                        />
                        <input type="text" placeholder="Paste URL from wowhead to import" v-model="talentImport" @input="importTalents">
                    </div>
                    <talent-calculator v-model="activePlayer.talents" :level="activePlayer.level" />
                </div>

                <div class="rotation" v-if="activeTab == 'rotation' && activePlayer">
                    <div class="form-box larger">
                        <div class="form-title">Rotation</div>
                        <apl v-model="activePlayer.apl" :player="activePlayer" @save="editAplOpen" />
                    </div>
                    <div class="form-boxes">
                        <div class="form-box medium apl-list">
                            <div class="form-title">Presets</div>
                            <div class="list">
                                <div
                                    class="item default"
                                    v-for="item in defaultApls()"
                                    :key="item.id"
                                    @click="selectApl(item)"
                                >
                                    <div class="name">{{ item.name }}</div>
                                </div>
                            </div>
                            <div class="form-item">
                                <select-simple
                                    v-model="playerModelCopy"
                                    :options="otherPlayerOptions"
                                    empty-option="Copy from..."
                                    @input="copyPlayerApl"
                                />
                            </div>
                        </div>
                        <div class="form-box medium apl-list" v-if="apls.length">
                            <div class="form-title">Your rotations</div>
                            <div class="list">
                                <div
                                    class="item default"
                                    v-for="item in apls"
                                    :key="item.id"
                                    @click="selectApl(item)"
                                >
                                    <div class="name">{{ item.name }}</div>
                                    <div class="actions">
                                        <button class="delete" @click.stop="deleteApl(item.id)">
                                            <micon icon="delete" />
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="export" v-if="activeTab == 'export'">
                    <div class="form-box">
                        <div class="title">Export</div>
                        <div class="form-item">
                            <label>What to export</label>
                            <select-simple v-model="exportType" :options="exportTypeOptions" />
                        </div>
                        <div class="buttons">
                            <button class="btn btn-primary" @click="exportSubmit">Export</button>
                            <span class="middle copy-success" v-if="exportSuccess">
                                <micon icon="check" />
                                <span class="middle">Copied to clipboard!</span>
                            </span>
                        </div>
                    </div>
                </div>

                <div class="import" v-if="activeTab == 'import'">
                    <div class="form-box large">
                        <div class="title">Import</div>
                        <div class="form-item">
                            <textarea v-model="importData" placeholder="Supported formats: MageSim, Sixtyupgrades, WowSimsExporter"></textarea>
                        </div>
                        <div class="buttons">
                            <button class="btn btn-primary" @click="importSubmit">Import</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="result-backdrop" @click="closeResult" v-if="resultOpen"></div>
        <div id="result" :class="{active: resultOpen}">
            <div class="result-content">
                <button class="close" @click="closeResult">
                    <micon icon="close" />
                </button>
                <template v-if="result">
                    <div class="tabs">
                        <button class="tab" :class="{active: activeResultTab == 'overview'}" @click="activeResultTab = 'overview'">
                            Overview
                        </button>
                        <template v-if="result.iterations">
                            <button class="tab" :class="{active: activeResultTab == 'histogram'}" @click="activeResultTab = 'histogram'">
                                Histogram
                            </button>
                        </template>
                        <template v-else>
                            <button class="tab" :class="{active: activeResultTab == 'log'}" @click="activeResultTab = 'log'">
                                Combat log
                            </button>
                            <button class="tab" :class="{active: activeResultTab == 'graph'}" @click="activeResultTab = 'graph'">
                                Graph
                            </button>
                        </template>
                    </div>

                    <div class="overview" v-if="activeResultTab == 'overview'">
                        <div class="dps-overview" v-if="resultOpen">
                            <div class="players">
                                <div class="player" v-for="player in result.players">
                                    <div class="progress-wrapper">
                                        <progress-circle :value="playerDps(player) / result.dps" :animate="true" />
                                        <div class="center">
                                            <div class="value">
                                                <animate-number :end="playerDps(player) / result.dps * 100" :decimals="0" />%
                                            </div>
                                        </div>
                                    </div>
                                    <div class="info">
                                        <div class="name">{{ player.name }}</div>
                                        <div class="dps">
                                            <animate-number :end="playerDps(player)" />
                                        </div>
                                    </div>
                                </div>
                                <div class="form-item" v-if="result.ignite_dps">
                                    <checkbox label="Normalize ignite"><input type="checkbox" v-model="settings.normalize_ignite"></checkbox>
                                </div>
                            </div>
                            <div class="total progress-wrapper">
                                <progress-circle :value="1" :animate="true" />
                                <div class="center">
                                    <div class="title">Total dps</div>
                                    <div class="value">
                                        <animate-number :end="result.dps" />
                                    </div>
                                    <div class="notice" v-if="result.iterations">{{ result.min_dps.toFixed() }} - {{ result.max_dps.toFixed() }}</div>
                                </div>
                            </div>
                            <div class="ignite progress-wrapper" v-if="result.ignite_dps">
                                <progress-circle :value="result.ignite_dps / result.dps" :animate="true" />
                                <div class="center">
                                    <div class="title">Ignite dps</div>
                                    <div class="value">
                                        <animate-number :end="result.ignite_dps" />
                                    </div>
                                </div>
                            </div>
                            <div class="info">
                                <table>
                                    <tbody>
                                        <tr><td>Players:</td><td>{{ result.players.length }}</td></tr>
                                        <tr><td>Iterations:</td><td>{{ result.iterations }}</td></tr>
                                        <tr><td>Execution time:</td><td>{{ result.time.toFixed(2) }}s</td></tr>
                                        <tr><td>Time / iteration:</td><td>{{ (result.time / result.iterations * 1000).toFixed(2) }}ms</td></tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>

                    <div class="histogram" v-if="activeResultTab == 'histogram'">
                        <div class="search">
                            <div class="search-histogram">
                                <select-simple v-model="filterHistogram" :options="filterHistogramOptions" :fill-missing="true" />
                            </div>
                        </div>
                        <histogram :data="histogramData" />
                    </div>

                    <div class="graph" v-if="activeResultTab == 'graph'">
                        <div class="search">
                            <div class="search-player">
                                <select-simple v-model="filterPlayer" :options="filterPlayerOptions" empty-option="All players" />
                            </div>
                        </div>
                        <combat-chart :result="result" :player="filterPlayer" />
                    </div>

                    <template v-if="result.iterations">

                    </template>
                    <template v-else>
                        <div class="combat-log" v-if="activeResultTab == 'log'">
                            <div class="search">
                                <div class="search-player">
                                    <select-simple v-model="filterPlayer" :options="filterPlayerOptions" empty-option="All players" />
                                </div>
                            </div>
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
                                            <template v-if="log.spell_result == 'Hit' || log.spell_result == 'Crit'">
                                                <span class="format-dmg" :class="['spell-result-'+css(log.spell_result)]">
                                                    {{ log.value.toFixed() }}
                                                </span>
                                                <span v-if="log.value2">
                                                    (-{{ log.value2.toFixed() }})
                                                </span>
                                            </template>
                                            <span v-else-if="log.log_type == 'Mana'">
                                                <span class="format-mana">{{ log.value.toFixed() }}</span>
                                            </span>
                                            <span v-else-if="log.value">
                                                {{ log.value.toPrecision(2) }}
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
        </div>

        <div class="notifications">
            <div
                class="notification"
                :class="[_.get(notification, 'class', null)]"
                v-for="notification in notifications"
            >
                <div class="title" v-if="notification.title" v-html="notification.title"></div>
                <div class="text" v-if="notification.text" v-html="notification.text"></div>
            </div>
        </div>

        <spotlight ref="raidEdit" class="small" v-slot="{ close }">
            <div class="default raid-edit">
                <div class="form-title">Create raid</div>
                <div class="form-item">
                    <label>Name</label>
                    <input type="text" v-model="raidModel.name" @keydown.enter="updateRaid">
                </div>
                <div class="form-item">
                    <label>Faction</label>
                    <select-simple v-model="raidModel.faction" :options="factionOptions" />
                </div>
                <div class="form-item">
                    <label>Copy from</label>
                    <select-simple v-model="raidModelCopy" :options="raidCopyOptions" empty-option="None" />
                </div>
                <div class="buttons">
                    <button class="btn btn-primary" @click="updateRaid">Save raid</button>
                    <button class="btn btn-secondary" @click="close">Cancel</button>
                </div>
            </div>
        </spotlight>

        <spotlight ref="playerEdit" class="small">
            <div class="default player-edit">
                <div class="form-title">Create player</div>
                <div class="form-item">
                    <label>Name</label>
                    <input type="text" v-model="playerModel.name" @keydown.enter="updatePlayer">
                </div>
                <div class="form-item">
                    <label>Race</label>
                    <select-simple v-model="playerModel.race" :options="raceOptions" />
                </div>
                <div class="form-item">
                    <label>Copy from</label>
                    <select-simple v-model="playerModelCopy" :options="playerCopyOptions" empty-option="None" />
                </div>
                <div class="buttons">
                    <button class="btn btn-primary" @click="updatePlayer">Save player</button>
                </div>
            </div>
        </spotlight>

        <spotlight ref="playerImport" class="small">
            <div class="default player-edit">
                <div class="form-title">Import player</div>
                <div class="form-item">
                    <label>Import to</label>
                    <select-simple v-model="playerModelCopy" :options="playerCopyOptions" empty-option="New player" />
                </div>
                <template v-if="playerModelCopy">
                    <div class="form-item">
                        <checkbox label="Import config"><input type="checkbox" v-model="playerImportConfig"></checkbox>
                    </div>
                    <div class="form-item">
                        <checkbox label="Import gear"><input type="checkbox" v-model="playerImportLoadout"></checkbox>
                    </div>
                </template>
                <template v-else>
                    <div class="form-item">
                        <label>Name</label>
                        <input type="text" v-model="playerModel.name" @keydown.enter="updatePlayer">
                    </div>
                </template>
                <div class="import-message" v-if="importMessage" v-html="importMessage"></div>
                <div class="buttons">
                    <button class="btn btn-primary" @click="updatePlayer">Save player</button>
                </div>
            </div>
        </spotlight>

        <spotlight ref="editApl" class="small">
            <div class="default apl-edit" v-if="aplModel">
                <div class="form-title">Save rotation</div>
                <div class="form-item">
                    <label>Name</label>
                    <input type="text" v-model="aplModel.name" @keydown.enter="updateApl">
                </div>
                <div class="form-item">
                    <label>Save as</label>
                    <select-simple
                        v-model="aplModel.id"
                        :options="aplTargetOptions"
                        empty-option="New rotation"
                    />
                </div>
                <div class="buttons">
                    <button class="btn btn-primary" @click="updateApl">Save rotation</button>
                </div>
            </div>
        </spotlight>

        <spotlight ref="confirmSpotlight" class="small confirm">
            <div class="default">
                <div class="confirm-text">{{ confirmation.text }}</div>
                <div class="buttons">
                    <button class="btn btn-primary" @click="confirmationContinue">{{ confirmation.confirm }}</button>
                    <button class="btn btn-secondary" @click="confirmationCancel">{{ confirmation.abort }}</button>
                </div>
            </div>
        </spotlight>

        <spotlight ref="alertSpotlight" class="small alert">
            <div class="default">
                <div class="alert-text">{{ alerter.text }}</div>
                <div class="buttons">
                    <button class="btn btn-primary" @click="alertClose">Close</button>
                </div>
            </div>
        </spotlight>
    </div>
</template>