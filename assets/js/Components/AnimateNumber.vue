<script setup>
import { ref, watch } from "vue";

const props = defineProps({
    start: {
        type: Number,
        default: 0,
    },
    end: {
        type: Number,
        required: true,
    },
    duration: {
        type: Number,
        default: 1000,
    },
    decimals: {
        type: Number,
        default: 1,
    },
    rounding: {
        type: String,
        default: "round", // round, floor, ceil
    },
    format: {
        type: String,
        default: "fixed", // round, fixed
    },
    easing: {
        type: String,
        default: "ease",
    },
});

const easeFn = {
    linear: (t) => {
        return t;
    },
    ease: (t) => {
        return t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
    },
    easeQuad: (t) => {
        return t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
    },
    easeInQuad: (t) => {
        return t * t;
    },
    easeOutQuad: (t) => {
        return 1 - (1 - t) * (1 - t);
    },
    easeCubic: (t) => {
        return t < 0.5 ? 4 * t * t * t : (t - 1) * (2 * t - 2) * (2 * t - 2) + 1;
    },
    easeInCubic: (t) => {
        return t * t * t;
    },
    easeOutCubic: (t) => {
        return (--t) * t * t + 1;
    },
};

const animatedNumber = ref(props.start);

const animate = () => {
    const elapsed = performance.now() - startTime
    const fn = easeFn.hasOwnProperty(props.easing) ? easeFn[props.easing] : easeFn.linear;
    const value = fn(Math.min(1, elapsed / props.duration));
    let nr = props.start + value * (props.end - props.start);
    if (props.format == "round")
        nr = nr.toPrecision(props.decimals);
    else if (props.format == "fixed")
        nr = nr.toFixed(props.decimals);
    animatedNumber.value = nr;
    if (value != 1)
        requestAnimationFrame(animate);
};

let startTime = null;
const start = () => {
    startTime = performance.now();
    requestAnimationFrame(animate);
};
defineExpose({ start });

watch(() => props.end, start);
start();
</script>

<template>
    <span class="animate-number">{{ animatedNumber }}</span>
</template>