<script setup>
const props = defineProps({
    icon: {
        type: String,
        required: true,
    },
    type: {
        type: String,
        default: "sharp",
    },
    weight: {
        type: Number,
        default: 0,
    },
});
</script>

<template>
    <span
        class="micon"
        :class="['material-symbols-'+props.type, props.weight ? 'weight-'+props.weight : '']"
    >{{ props.icon }}</span>
</template>