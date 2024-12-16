<script setup>
import { ref, reactive, computed, onMounted, onUnmounted } from "vue";
import _ from "lodash";
import apl from "../apl";
import common from "../common";

const props = defineProps(["modelValue", "player"]);
const emits = defineEmits(["update:modelValue"]);

/*
 * Collapse
 */
const collapsed = ref([]);
const isCollapsed = (id) => {
    return collapsed.value.includes(id);
};
const collapseToggle = (id) => {
    if (isCollapsed(id))
        collapsed.value = collapsed.value.filter(i => i !== id);
    else
        collapsed.value.push(id);
};
const collapseAll = () => {
    if (collapsed.value.length === 0)
        collapsed.value = props.modelValue.items.map(item => item.id);
    else
        collapsed.value = [];
};

/*
 * Item
 */
const createItem = () => {
    let item = apl.item();
    props.modelValue.items.push(item);
    changed();
};
const deleteItem = (id) => {
    props.modelValue.items = props.modelValue.items.filter(item => item.id !== id);
    changed();
};
const copyItem = (item) => {
    let copy = _.cloneDeep(item);
    copy.id = common.uuid();
    let index = props.modelValue.items.findIndex(i => i.id === item.id);
    props.modelValue.items.splice(index, 0, copy);
    changed();
};
const statusToggle = (item) => {
    item.status = !item.status;
    changed();
};
const itemTitle = (item) => {
    let arr = [];

    let action = apl.actions().find(a => a.type == item.action.action_type && a.key == item.action.key);
    if (action) {
        arr.push(action.title);
    }

    if (item.action.action_type == apl.action_type.SEQUENCE)
        arr.push(item.action.sequence.length+" actions");

    let numCond = (cond) => {
        if (cond.condition_type == apl.condition_type.NONE)
            return 0;
        let n = 1;
        for (let c of cond.conditions)
            n+= numCond(c);
        return n;
    };

    let n = numCond(item.condition);
    if (n == 1)
        arr.push(n+" condition");
    else if (n > 1)
        arr.push(n+" conditions");

    return arr.join(", ");
};

/*
 * Drag move
 */
const el = ref();
const dragging = reactive({
    id: null,
    index: 0,
    start: 0,
    x: 0,
    y: 0,
})
const onDragStart = (e, id) => {
    if (props.modelValue.items.length < 2)
        return;
    dragging.id = id;
    dragging.index = props.modelValue.items.findIndex(item => item.id === id);
    dragging.start = dragging.index;
};
const onDragEnd = () => {
    if (!dragging.id)
        return;

    if (dragging.start !== dragging.index) {
        let item = props.modelValue.items.splice(dragging.start, 1)[0];
        props.modelValue.items.splice(dragging.index > dragging.start ? dragging.index-1 : dragging.index, 0, item);
        changed();
    }

    dragging.id = null;
};
const onDragMove = (e) => {
    if (!dragging.id)
        return;
    dragging.x = e.clientX;
    dragging.y = e.clientY;
    if (el.value) {
        let wrapper = el.value.children[0];
        for (var i=0; i<wrapper.children.length; i++) {
            let item = wrapper.children[i];
            let rect = item.getBoundingClientRect();
            if (e.clientY < rect.top + rect.height/2) {
                dragging.index = i;
                break;
            }
        }
        if (i == wrapper.children.length)
            dragging.index = props.modelValue.items.length;
    }
};
const isDragTo = (index) => {
    return dragging.id && dragging.index == index && dragging.start != index && dragging.start != index-1;
};
const isDragEnd = computed(() => {
    return dragging.id && dragging.index == props.modelValue.items.length && dragging.start != dragging.index-1;
});

/*
 * Update
 */
const changed = () => {
    emits("update:modelValue", props.modelValue);
};

/*
 * Events
 */
onMounted(() => {
    window.addEventListener("mousemove", onDragMove);
    window.addEventListener("mouseup", onDragEnd);
});
onUnmounted(() => {
    window.removeEventListener("mousemove", onDragMove);
    window.removeEventListener("mouseup", onDragEnd);
});
</script>

<template>
    <div class="apl" ref="el">
        <button class="btn btn-secondary small collapse-all" @click="collapseAll">
            <template v-if="collapsed.length == 0">Collapse all</template>
            <template v-else>Expand all</template>
        </button>
        <div class="apl-items" :class="{dragend: isDragEnd}">
            <div
                class="apl-item"
                :class="{collapsed: isCollapsed(item.id), dragto: isDragTo(index), dragfrom: dragging.id == item.id}"
                v-for="(item, index) in modelValue.items"
                :key="item.id"
            >
                <div class="header">
                    <button class="toggle" @click="collapseToggle(item.id)">
                        <span v-if="isCollapsed(item.id)">
                            <micon icon="add" />
                            <tooltip>Expand</tooltip>
                        </span>
                        <span v-else>
                            <micon icon="remove" />
                            <tooltip>Collapse</tooltip>
                        </span>
                    </button>
                    <button class="copy" @click="copyItem(item)">
                        <micon icon="content_copy" />
                        <tooltip>Clone</tooltip>
                    </button>
                    <button class="status" @click="statusToggle(item)">
                        <template v-if="item.status">
                            <micon icon="visibility" />
                            <tooltip>Disable</tooltip>
                        </template>
                        <template v-else>
                            <micon icon="visibility_off" />
                            <tooltip>Enable</tooltip>
                        </template>
                    </button>
                    <button class="delete" @click="deleteItem(item.id)">
                        <micon icon="delete" />
                        <tooltip>Delete</tooltip>
                    </button>
                </div>

                <div class="drag" @mousedown="onDragStart($event, item.id)">
                    <micon icon="drag_indicator" />
                </div>

                <div class="title" v-if="isCollapsed(item.id)" @click="collapseToggle(item.id)">
                    {{ itemTitle(item) }}
                </div>
                <div class="body" v-else>
                    <apl-condition v-model="item.condition" :player="props.player" />
                    <apl-action v-model="item.action" :player="props.player" />
                </div>
            </div>
        </div>
        <button class="btn btn-primary" @click="createItem">New action</button>
        <div class="dragger" v-if="dragging.id" :style="{transform: 'translate3d('+dragging.x+'px,'+dragging.y+'px,0)'}"></div>
    </div>
</template>