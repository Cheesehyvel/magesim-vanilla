<script setup>
const props = defineProps({
    icon: {
        type: String,
        default: "help"
    },
    position: {
        type: String,
        default: "top"
    },
});
</script>

<template>
    <div class="help">
        <micon :icon="icon" />
        <tooltip :position="position"><slot></slot></tooltip>
    </div>
</template>