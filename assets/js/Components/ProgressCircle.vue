<script setup>
import { computed } from "vue";

const props = defineProps(["value"]);

const progressStyle = computed(() => {
    return {
        strokeDashoffset: 100 * (1.0 - props.value),
    }
});
</script>

<template>
    <div class="progress-circle">
        <svg width="120" height="120" viewBox="0 0 120 120">
            <circle cx="60" cy="60" r="53" fill="none" stroke="#000116" stroke-width="14"></circle>
            <circle class="percent" cx="60" cy="60" r="53" fill="none" stroke="currentColor" stroke-width="14" pathLength="100" :style="progressStyle"></circle>
        </svg>
    </div>
</template>