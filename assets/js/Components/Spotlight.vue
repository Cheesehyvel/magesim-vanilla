<script setup>
import { ref, reactive, computed, nextTick, onUnmounted } from "vue";

const props = defineProps({
    valign: {
        type: Boolean,
        default: true,
    },
});

const emit = defineEmits(["open", "close"]);

const el = ref();
const light = ref();
const isOpen = ref(false);
const isLoaded = ref(false);
const lightStyle = reactive({ top: "0" });

const moveDOM = () => {
    if (el.value.parentNode != document.body) {
        el.value.parentNode.removeChild(el.value);
        document.body.appendChild(el.value);
    }
};

const adjust = () => {
    let scrollTop = (document.documentElement && document.documentElement.scrollTop || document.body && document.body.scrollTop || 0);
    lightStyle.top = (scrollTop + Math.max(0, ((window.innerHeight - light.value.offsetHeight)/2)))+"px";
};

const focus = () => {
    let inp = el.value.querySelector("input");
    if (!inp)
        inp = inp.value.querySelector("textarea");
    if (inp) {
        inp.focus();
        nextTick(() => {
            inp.focus();
        });
    }
};

const open = (do_focus) => {
    moveDOM();
    isOpen.value = true;
    emit("open");
    if (props.valign) {
        nextTick(() => {
            adjust();
            nextTick(() => {
                adjust();
            });
            if (do_focus) {
                nextTick(() => {
                    focus();
                });
            }
        });
    }
};

const close = () => {
    isOpen.value = false;
    emit("close");
};

onUnmounted(() => {
    if (isOpen.value && el.value)
        el.value.parentNode.removeChild(el.value);
});

defineExpose({
    open,
    close,
    focus,
    adjust
});
</script>


<template>
    <div 
        ref="el"
        class="spotlight"
        :class="{open: isOpen, valign: props.valign}"
    >
        <div class="dark" @click="close"></div>
        <div class="light" :style="lightStyle" ref="light">
            <div class="content">
                <slot :close="close"></slot>
            </div>
        </div>
    </div>
</template>