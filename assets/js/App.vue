<script setup>
import SimContainer from "./sim_container";
import { computed, ref, reactive } from "vue";
import common from "./common";
import _ from "lodash";

const baseStats = (race, level) => {
    let stats = {
        int: 0,
        spi: 0,
        mp5: 0,
        crit: 0,
        hit: 0,
        sp: 0,
        sp_arcane: 0,
        sp_fire: 0,
        sp_frost: 0,
        sp_nature: 0,
        sp_shadow: 0,
        spell_penetration: 0,
    };

    if (race == common.races.Gnome) {
        stats.int = 132;
        stats.spi = 120;
    }
    else if (race == common.races.Human) {
        stats.int = 125;
        stats.spi = 126;
    }
    else if (race == common.races.Troll) {
        stats.int = 121;
        stats.spi = 121;
    }
    else if (race == common.races.Undead) {
        stats.int = 123;
        stats.spi = 125;
    }

    // TODO: REMOVE THIS, ONLY FOR TESTING
    stats.crit = 50;

    return stats;
};

const baseTalents = () => {
    let talents = new Array(64).fill(0);
    
    talents[1] = 2;
    talents[2] = 3;
    talents[6] = 5;
    talents[7] = 2;
    talents[8] = 3;
    talents[12] = 2;
    talents[17] = 5;
    talents[19] = 5;
    talents[21] = 3;
    talents[22] = 2;
    talents[24] = 1;
    talents[26] = 3;
    talents[28] = 3;
    talents[29] = 3;
    talents[31] = 5;
    talents[32] = 1;
    talents[35] = 3;

    return talents;
};

let player_id = 1;
const basePlayer = () => {
    return {
        id: player_id++,
        race: common.races.Gnome, 
        talents: baseTalents(),
        stats: baseStats(common.races.Gnome),
        level: 60,
        mage_armor: true,
        mana_spring: false,
        imp_mana_spring: false,
        dmf_dmg: false,
    }
};

const default_config = {
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
    players: [
        basePlayer(),
        basePlayer(),
        basePlayer(),
        basePlayer(),
    ],
};

const config = reactive(_.cloneDeep(default_config));
const iterations = ref(5000);
const threads = ref(navigator.hardwareConcurrency);
const result = ref(null);
const is_running = ref(false);

const runSingle = () => {
    const sc = new SimContainer(threads.value, 1, r => {
        is_running.value = false;
        result.value = r;
        console.log(r);
    }, e => {
        console.error("Error", e);
    });

    is_running.value = true;
    result.value = null;
    sc.start(config);
};

const runMultiple = () => {
    const sc = new SimContainer(threads.value, iterations.value, r => {
        is_running.value = false;
        result.value = r;
        console.log(r);
    }, e => {
        console.error("Error", e);
    });

    is_running.value = true;
    result.value = null;
    sc.start(config);
};

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

const css = (str) => {
    return _.kebabCase(str);
};

const logTypes = ref([
    "CastSuccess", "SpellImpact", "Wait", "AuraGain", "AuraExpire",
]);

const filteredLog = computed(() => {
    if (!result.value)
        return [];

    return result.value.log.filter(log => logTypes.value.indexOf(log.log_type) != -1);
});

</script>

<template>
    <div class="app">
        <div>MageSim Rust: Vanilla</div>

        <div style="margin: 20px 0">
            <input type="text" v-model.number="config.duration" style="width: 50px" :disabled="is_running"> duration<br>
            <input type="text" v-model.number="iterations" style="width: 50px" :disabled="is_running"> iterations<br>
            <input type="text" v-model.number="threads" style="width: 50px" :disabled="is_running"> threads<br><br>
            <button type="button" class="btn-primary" @click="runMultiple" :disabled="is_running">Run multiple</button>&nbsp;
            <button type="button" class="btn-secondary" @click="runSingle" :disabled="is_running">Run single</button>
        </div>

        <template v-if="result">
            <div v-if="result.time" style="margin: 0 0 10px;">Completed {{ result.iterations }} in {{ result.time.toFixed(2) }}s</div>
            <template v-if="result.iterations">
                <div><b>Total dps: {{ result.avg_dps.toFixed(2) }} ({{ result.min_dps.toFixed() }} - {{ result.max_dps.toFixed() }})</b></div>
                <div v-for="(dps, index) in result.player_avg_dps">
                    <div>Player {{ index+1 }} dps: {{ dps.toFixed(2) }} ({{ result.player_min_dps[index].toFixed() }} - {{ result.player_max_dps[index].toFixed() }})</div>
                </div>
            </template>
            <template v-else>
                <div><b>Total: {{ result.dps.toFixed(2) }} dps ({{ result.dmg }} dmg)</b></div>
                <div v-for="(dps, index) in result.player_dps">
                    <div>Player {{ index+1 }}: {{ dps.toFixed(2) }} dps ({{ result.player_dmg[index] }} dmg)</div>
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