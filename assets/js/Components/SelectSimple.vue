<script setup>
import { ref, computed, onUnmounted, watch, nextTick } from "vue";
const emit = defineEmits(["update:modelValue", "input"]);
const props = defineProps(["modelValue", "options", "emptyOption", "placeholder", "fillMissing"]);

const el = ref();
const emptyOption = computed(() => {
    if (props.emptyOption === true)
        return "- Choose -";
    if (props.emptyOption && props.emptyOption !== true)
        return props.emptyOption;
     return false;
});

const optionTitle = (value) => {
    let opt = props.options.find(v => v.value === value);
    if (opt)
        return opt.title;
    if (emptyOption.value)
        return emptyOption.value;
    if (props.placeholder)
        return props.placeholder;
    return null;
};

const inputValue = ref(optionTitle(props.modelValue));

const input = (value) => {
    inputValue.value = optionTitle(value);
    hideOptions();
    emit("update:modelValue", value);
    emit("input", value);
};

const showingOptions = ref(false);
const hideOptions = () => {
    showingOptions.value = false;
};
const toggleOptions = () => {
    showingOptions.value = !showingOptions.value;

    if (showingOptions.value)
        nextTick(checkCollision);
};

const isEmpty = computed(() => {
    return props.modelValue === null || props.modelValue === undefined;
});
const fillMissing = () => {
    if (!props.fillMissing)
        return;
    let opt = props.options.find(v => v.value === props.modelValue);
    if (!opt && emptyOption.value === false && !props.placeholder && props.options.length) {
        emit("update:modelValue", props.options[0].value);
        emit("input", props.options[0].value);
    }
};

const elOptions = ref();
const optionsPos = ref("bottom");
const checkCollision = () => {
    if (!elOptions.value || !el.value)
        return;
    let elRect = el.value.getBoundingClientRect();
    let optRect = elOptions.value.getBoundingClientRect();
    if (elRect.bottom + optRect.height > document.body.offsetHeight)
        optionsPos.value = "top";
    else
        optionsPos.value = "bottom";
};

const onWindowClick = (e) => {
    if (e.target && el.value && !el.value.contains(e.target))
        hideOptions();
};

onUnmounted(() => {
    window.removeEventListener("click", onWindowClick);
});
window.addEventListener("click", onWindowClick);

watch(() => props.modelValue, (value) => {
    inputValue.value = optionTitle(value);
    fillMissing();
});
watch(() => props.options, (value) => {
    inputValue.value = optionTitle(props.modelValue);
    fillMissing();
});

fillMissing();
</script>

<template>
    <div class="select-simple" :class="{open: showingOptions, empty: isEmpty}" ref="el">
        <div class="input" @click="toggleOptions">
            <div class="textfield"><span>{{ inputValue }}</span></div>
            <div class="icon">
                <micon icon="keyboard_arrow_down" />
            </div>
        </div>
        <div class="options" :class="['pos-'+optionsPos]" v-if="showingOptions" ref="elOptions">
            <div class="option empty" v-if="emptyOption !== false" @click="input(null)">{{ emptyOption }}</div>
            <div
                class="option"
                 :class="{active: option.value === props.modelValue}"
                 v-for="option in props.options"
                 :key="option.value"
                 @click="input(option.value)"
             >{{ option.title }}</div>
        </div>
    </div>
</template>