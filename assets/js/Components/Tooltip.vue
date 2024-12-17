<script setup>
import { ref, onMounted, onUpdated, nextTick } from "vue";

const props = defineProps({
    position: {
        type: String,
        default: "top"
    },
});

const el = ref(null);
const pos = ref(props.position);

const checkCollision = () => {
    if (!el.value)
        return;
    var rect = el.value.getBoundingClientRect();
    if (pos.value == "right" && rect.x + rect.width > document.body.offsetWidth)
        pos.value = pos.value.replace("right", "left");
    else if (pos.value == "left" && rect.x < 0)
        pos.value = pos.value.replace("left", "right");
};

onMounted(() => {
    el.value.parentNode.classList.add("tooltip-anchor");
    el.value.parentNode.addEventListener("mouseenter", checkCollision);
});

onUpdated(() => {
    if (!el.value.parentNode.classList.contains("tooltip-anchor")) {
        el.value.parentNode.classList.add("tooltip-anchor");
        el.value.parentNode.addEventListener("mouseenter", checkCollision);
    }
});
</script>

<template>
    <div ref="el" class="tooltip" :class="['position-'+pos]">
        <slot></slot>
    </div>
</template>