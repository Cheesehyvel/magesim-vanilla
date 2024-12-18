<script setup>
const props = defineProps({
    icon: {
        type: String,
        default: "check",
    },
    label: {
        type: String,
    },
    tip: {
        default: null
    }
});
</script>

<template>
    <label class="checkbox">
        <slot />
        <span class="checkbox-inner">
            <span class="checkbox-icon middle" v-if="props.icon"><micon :icon="props.icon" :weight="700" /></span>
            <span class="checkbox-label middle">{{ label }}</span>
            <tooltip v-if="props.tip">{{ props.tip }}</tooltip>
        </span>
    </label>
</template>