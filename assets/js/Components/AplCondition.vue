<script setup>
import { ref, computed } from "vue";
import apl from "../apl";

const props = defineProps(["modelValue", "deletable", "player"]);
const emits = defineEmits(["update:modelValue", "delete"]);

const typeOptions = [
    { value: apl.condition_type.NONE, title: "No condition" },
    { value: apl.condition_type.CMP, title: "Compare" },
    { value: apl.condition_type.NOT, title: "Not" },
    { value: apl.condition_type.TRUE, title: "Is true" },
    { value: apl.condition_type.FALSE, title: "Is false" },
    { value: apl.condition_type.AND, title: "All of" },
    { value: apl.condition_type.OR, title: "Any of" },
];
const opOptions = [
    { value: apl.condition_op.EQ, title: "=" },
    { value: apl.condition_op.NEQ, title: "≠" },
    { value: apl.condition_op.GT, title: ">" },
    { value: apl.condition_op.GTE, title: "≥" },
    { value: apl.condition_op.LT, title: "<" },
    { value: apl.condition_op.LTE, title: "≤" },
];

const onTypeInput = (type) => {
    let conditions = 0, values = 0;
    if (type == apl.condition_type.NOT)
        conditions = 1;
    if (type == apl.condition_type.CMP)
        values = 2;
    if (type == apl.condition_type.TRUE || type == apl.condition_type.FALSE)
        values = 1;
    if (type == apl.condition_type.AND || type == apl.condition_type.OR)
        conditions = Math.max(2, props.modelValue.conditions.length);

    if (props.modelValue.conditions.length < conditions) {
        for (let i = props.modelValue.conditions.length; i < conditions; i++)
            props.modelValue.conditions.push(apl.condition());
    }
    else {
        props.modelValue.conditions.splice(conditions);
    }

    if (props.modelValue.values.length < values) {
        for (let i = props.modelValue.values.length; i < values; i++)
            props.modelValue.values.push(apl.value());
    }
    else {
        props.modelValue.values.splice(values);
    }

    changed();
};

const expectValue = computed(() => {
    if (props.modelValue.type == apl.condition_type.CMP)
        return "vfloat";
    if (props.modelValue.type == apl.condition_type.TRUE || props.modelValue.type == apl.condition_type.FALSE)
        return "bool";
    return null;
});
const hasOp = computed(() => {
    return props.modelValue.type == apl.condition_type.CMP;
});

/*
 * Create / delete
 */
const canCreateCondition = computed(() => {
    return props.modelValue.type == apl.condition_type.AND || props.modelValue.type == apl.condition_type.OR;
});
const createCondition = () => {
    props.modelValue.conditions.push(apl.condition());
    changed();
};
const deleteCondition = (index) => {
    props.modelValue.conditions.splice(index, 1);
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
    <div class="apl-condition">
        <div class="values" :class="['values-'+props.modelValue.values.length]">
            <select-simple
                v-model="props.modelValue.type"
                :options="typeOptions"
                @input="onTypeInput"
            />
            <apl-value v-model="props.modelValue.values[0]" :player="props.player" :expect="expectValue" v-if="modelValue.values.length > 0" />
            <select-simple
                class="op"
                v-model="props.modelValue.op"
                :options="opOptions"
                @input="changed"
                v-if="hasOp"
            />
            <apl-value v-model="props.modelValue.values[1]" :player="props.player" :expect="expectValue" v-if="modelValue.values.length > 1" />
        </div>

        <template v-for="(condition, index) in props.modelValue.conditions" :key="condition.id">
            <apl-condition v-model="props.modelValue.conditions[index]" :player="props.player" :deletable="true" @delete="deleteCondition(index)" />
        </template>

        <button class="btn btn-secondary small" @click="createCondition" v-if="canCreateCondition">Add condition</button>

        <button class="delete" @click="emits('delete')" v-if="props.deletable">
            <micon icon="delete" />
        </button>
    </div>
</template>