<script setup>
import { ref, computed, onMounted, nextTick } from "vue";

const props = defineProps(["value", "animate"]);
const loaded = ref(props.animate ? false : true);

const progressStyle = computed(() => {
    return {
        strokeDashoffset: loaded.value ? 100 * (1.0 - props.value) : 100,
    }
});

if (props.animate) {
    onMounted(() => {
        setTimeout(() => {
            loaded.value = true;
        }, 10);
    });
}
</script>

<template>
    <div class="progress-circle">
        <svg width="120" height="120" viewBox="0 0 120 120">
            <circle class="background" cx="60" cy="60" r="53" fill="none" stroke="#000116" stroke-width="14"></circle>
            <circle class="percent" cx="60" cy="60" r="53" fill="none" stroke="currentColor" stroke-width="14" pathLength="100" :style="progressStyle"></circle>
        </svg>
    </div>
</template>