<script setup>
import { ref, computed } from "vue";
import apl from "../apl";
import { mage as talentTree } from "../talents";
import common from "../common";
import items from "../items";

const props = defineProps(["modelValue", "deletable", "player"]);
const emits = defineEmits(["update:modelValue", "delete"]);

const talentNames = talentTree.trees.reduce((a, b) => { return [...a, ...b.talents.rows.flat()]; }, []).map(t => t.name);
const playerItems = computed(() => {
    return _.values(props.player.loadout).map(i => i.item_id);
});
const filterOptions = (options) => {
    let isActive = (opt) => {
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
        if (opt.key == "Sequence" && props.deletable)
            return false;
        return true;
    };

    for (let option of options) {
        if (option.item) {
            let item = items.gear[option.title].find(i => i.id == option.item);
            if (item)
                option.title = _.upperFirst(option.title.split("_").join(" "))+": "+item.title;
        }
        if (!isActive(option))
            option.title+= " (inactive)";
    }

    return options;
};
const actionOptions = computed(() => {
    let options = apl.actions();
    for (let option of options)
        option.value = option.key;
    return filterOptions(options);
});
const changeAction = (value) => {
    if (value == "Sequence")
        props.modelValue.sequence = [apl.action()];
    else if (props.modelValue.sequence.length)
        props.modelValue.sequence = [];
    changed();
};

/*
 * Create / delete
 */
const createAction = () => {
    props.modelValue.sequence.push(apl.action());
    changed();
};
const deleteAction = (index) => {
    props.modelValue.sequence.splice(index, 1);
    changed();
};

/*
 * Update
 */
const changed = () => {
    emits("update:modelValue", props.modelValue);
};
</script>

<template>
    <div class="apl-action">
        <select-simple
            v-model="props.modelValue.key"
            :options="actionOptions"
            :fill-missing="true"
            @input="changeAction"
        />
        <div class="apl-sequence" v-if="props.modelValue.key == 'Sequence'">
            <template v-for="(action, index) in props.modelValue.sequence" :key="action.id">
                <apl-action
                    v-model="props.modelValue.sequence[index]"
                    :player="props.player"
                    :deletable="true"
                    @delete="deleteAction(index)"
                    @update:modelValue="changed"
                />
            </template>
            <button class="btn btn-secondary small" @click="createAction">Add action</button>
        </div>

        <button class="delete" @click="emits('delete')" v-if="props.deletable">
            <micon icon="delete" />
        </button>
    </div>
</template>