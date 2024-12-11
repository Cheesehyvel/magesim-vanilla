<script setup>
import { computed } from "vue";

const emit = defineEmits(["update:modelValue"]);

const props = defineProps({
    modelValue: {
        default: null,
    },
    name: {
        type: String,
    },
    order: {
        type: String,
        default: "asc",
    }
});

const active = computed(() => {
    return props.modelValue && props.modelValue.name && props.modelValue.name == props.name;
});

const desc = computed(() => {
    return active.value && _.get(props.modelValue, "order", props.order) == "desc";
});

const icon = computed(() => {
    return desc.value ? "keyboard_arrow_up" : "keyboard_arrow_down";
});

const flipOrder = (order) => {
    return order == "asc" ? "desc" : "asc";
};

const onClick = () => {
    let value = {
        name: props.name,
        order: props.order,
    };

    if (_.get(props.modelValue, "name") == props.name)
        value.order = flipOrder(_.get(props.modelValue, "order", props.order));

    emit("update:modelValue", value);
};
</script>

<template>
    <span class="sort-link" :class="{active}" @click="onClick">
        <span><slot></slot></span>
        <micon :icon="icon" />
    </span>
</template>