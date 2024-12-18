<script setup>
import { ref, computed, onMounted, nextTick, watch } from "vue";
import { mage as talentTree } from "../talents";

const props = defineProps(["modelValue", "level"]);
const emits = defineEmits(["update:modelValue"]);

const maxTalents = props.level ? Math.max(0, props.level - 9) : 51;
const maxTalentPoints = (talent) => {
    return talent.spellIds.length;
};
const totalPoints = computed(() => {
    return props.modelValue.reduce((a,b) => a+b, 0);
});
const remainingPoints = computed(() => {
    return maxTalents - totalPoints.value;
});
const talentTreeFlat = computed(() => {
    let arr = [];
    for (let tree of talentTree.trees)
        arr = [...arr, ...tree.talents.rows.flat()];
    return arr;
});
const treePoints = (tree) => {
    let start = 0, num;
    for (let t in talentTree.trees) {
        num = talentTree.trees[t].talents.rows.flat().length;
        if (talentTree.trees[t].name == tree.name)
            return props.modelValue.slice(start, start+num).reduce((a,b) => a+b, 0);
        start+= num;
    }
    return 0;
};
const treeFromTalent = (talent) => {
    for (let tree of talentTree.trees) {
        if (tree.talents.rows.flat().find(t => t.name == talent.name))
            return tree;
    }
    return null;
};
const talentRowIndex = (talent, tree) => {
    if (!tree)
        tree = treeFromTalent(talent);
    for (let r in tree.talents.rows) {
        if (tree.talents.rows[r].find(t => t.name == talent.name))
            return r;
    }
    return 0;
};
const talentPoints = (talent) => {
    let index = _.findIndex(talentTreeFlat.value, {name: talent.name});
    if (index == -1)
        return 0;
    return props.modelValue[index];
};
const isTalentActive = (talent) => {
    let points = talentPoints(talent);
    // no points left
    if (totalPoints.value >= maxTalents && (points == 0 || points == maxTalentPoints(talent)))
        return false;
    // is maxed out
    if (points >= maxTalentPoints(talent))
        return false;
    // not enough points in tree
    let tree = treeFromTalent(talent);
    let r = talentRowIndex(talent, tree);
    if (treePoints(tree) < r * 5)
        return false;
    return true;
};
const talentUrl = (talent) => {
    let points = talentPoints(talent);
    return "https://www.wowhead.com/classic/spell="+talent.spellIds[points ? points-1 : 0];
};
const clickTalent = (talent, rightclick) => {
    let index = _.findIndex(talentTreeFlat.value, {name: talent.name});
    if (index == -1)
        return 0;

    let points = talentPoints(talent);
    let newPoints = points + (rightclick ? -1 : 1);
    if (points > maxTalentPoints(talent) || newPoints < 0)
        return;

    if (rightclick) {
        let tree = treeFromTalent(talent);
        // Find the start position of the tree in a flattened structure
        let start = 0;
        for (let t of talentTree.trees) {
            if (t.name == tree.name)
                break;
            start+= t.talents.rows.flat().length;
        }
        // Check if there are any points further down the tree
        let i = start;
        let trow = 10;
        let pointsPrev = 0, pointsCur = 0;
        for (let r = 0; r<tree.talents.rows.length; r++) {
            for (let t in tree.talents.rows[r]) {
                if (tree.talents.rows[r][t].name == talent.name)
                    trow = r;
                if (r > trow) {
                    if (props.modelValue[i] > 0) {
                        // The talent point in question is required for another row, so we cant remove it
                        if (pointsPrev <= r * 5)
                            return;
                        // The talent in question is required by another, so we cant remove it
                        if (tree.talents.rows[r][t].requires && tree.talents.rows[r][t].requires == talent.name)
                            return;
                    }
                }
                pointsCur+= props.modelValue[i];
                i++;
            }
            pointsPrev+= pointsCur;
            pointsCur = 0;
        }
    }
    else {
        if (!isTalentActive(talent))
            return;
        if (totalPoints.value >= maxTalents)
            return false;
    }

    let arr = props.modelValue;
    arr[index] = newPoints;
    emits("update:modelValue", arr);
};
const resetTree = (tree) => {
    let arr = props.modelValue;

    let start = 0, end;
    for (let t in talentTree.trees) {
        end = start + talentTree.trees[t].talents.rows.flat().length;
        if (talentTree.trees[t].name == tree.name) {
            for (let i = start; i<end; i++)
                arr[i] = 0;
            break;
        }
        start = end;
    }

    emits("update:modelValue", arr);
};
const reset = () => {
    emits("update:modelValue", new Array(maxTalents).fill(0));
};

const requiredClass = (talent) => {
    let cname = "";
    if (talent.requires) {
        let tree = treeFromTalent(talent);
        let dx, dy, req_x, req_y;
        for (let r in tree.talents.rows) {
            let x = 0;
            for (let t in tree.talents.rows[r]) {
                x++;
                if (tree.talents.rows[r][t].skip)
                    x+= tree.talents.rows[r][t].skip;
                if (tree.talents.rows[r][t].name == talent.requires) {
                    req_x = x;
                    req_y = r;
                }
                if (tree.talents.rows[r][t].name == talent.name) {
                    dx = x - req_x;
                    dy = r - req_y;
                    break;
                }
            }
            if (dx !== undefined)
                break;
        }

        if (dx !== undefined) {
            if (dy > 0) {
                cname = "up-"+dy;
                if (dx < 0)
                    cname+= "-left-"+Math.abs(dx);
                else if (dx > 0)
                    cname+= "-right-"+dx;
            }
        }
    }
    return cname;
};

const wowheadUrl = computed(() => {
    let url = "https://www.wowhead.com/classic/talent-calc/"+talentTree.class+"/";
    for (let t in talentTree.trees) {
        let str = talentTree.trees[t].talents.rows.flat().map(t => talentPoints(t)).join("");
        str = str.replace(/[0]+$/, "");
        url+= str+"-";
    }
    url = url.replace(/[-]+$/, "");
    return url;
});

const refreshTooltips = () => {
    if (window.$WowheadPower) {
        window.$WowheadPower.refreshLinks();
        nextTick(window.$WowheadPower.refreshLinks);
    }
};

onMounted(refreshTooltips);
watch(() => props.modelValue, refreshTooltips);
</script>

<template>
    <div class="talent-calculator class-mage" :class="{complete: remainingPoints <= 0}">
        <div class="header">
            <div class="icon"><img :src="talentTree.icon" alt=""></div>
            <div class="name">{{ talentTree.class }}</div>
            <div class="link">
                <a :href="wowheadUrl" target="_blank">
                    <micon icon="link" />
                    <span class="middle">Link</span>
                </a>
            </div>
            <div class="link">
                <button @click="reset">
                    <micon icon="close" />
                    <span class="middle">Reset</span>
                </button>
            </div>
            <div class="points" :class="{empty: totalPoints == 0}">Points left: {{ remainingPoints }}</div>
        </div>
        <div class="trees">
            <div class="tree" v-for="(tree, t) in talentTree.trees" :key="tree.name">
                <div class="header">
                    <div class="icon"><img :src="tree.icon" alt=""></div>
                    <div class="name">{{ tree.name }}</div>
                    <div class="points" :class="{empty: treePoints(tree) == 0}">{{ treePoints(tree) }}</div>
                </div>
                <div class="talents" :style="{backgroundImage: 'url('+tree.background+')'}">
                    <div class="row" v-for="(row, r) in tree.talents.rows">
                        <template v-for="(talent, c) in row">
                            <div class="talent none" v-for="i in talent.skip ? talent.skip : 0"></div>
                            <div
                                class="talent"
                                :class="{
                                    empty: talentPoints(talent) == 0,
                                    active: isTalentActive(talent),
                                    max: talentPoints(talent) == maxTalentPoints(talent),
                                }"
                                @click="clickTalent(talent)"
                                @contextmenu.prevent="clickTalent(talent, true)"
                            >
                                <a
                                    data-wh-icon-size="medium"
                                    data-wh-rename-link="false"
                                    data-whtticon="false"
                                    :href="talentUrl(talent)"
                                    @click.prevent=""
                                    target="_blank"
                                ></a>
                                <div class="num">{{ talentPoints(talent) }}/{{ maxTalentPoints(talent) }}</div>
                                <div class="required" v-if="talent.requires" :class="requiredClass(talent)"></div>
                            </div>
                        </template>
                    </div>
                </div>
                <div class="reset">
                    <button class="btn btn-text block" @click="resetTree(tree)">Reset</button>
                </div>
            </div>
        </div>
    </div>
</template>