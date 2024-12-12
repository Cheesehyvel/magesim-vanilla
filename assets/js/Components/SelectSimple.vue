<script setup>
import { ref, computed, onUnmounted, watch } from "vue";
const emit = defineEmits(["update:modelValue", "input"]);
const props = defineProps(["modelValue", "options", "emptyOption", "placeholder"]);

const el = ref();
const emptyOption = computed(() => {
    if (props.emptyOption === false)
        return false;
    if (props.emptyOption && props.emptyOption !== true)
        return props.emptyOption;
     return "- Choose -";
});

const optionTitle = (value) => {
    let opt = _.find(props.options, {value: value});
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
};

const isEmpty = computed(() => {
    return props.modelValue === null || props.modelValue === undefined;
});

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
});
</script>

<template>
    <div class="select-simple" :class="{open: showingOptions, empty: isEmpty}" ref="el">
        <div class="input" @click="toggleOptions">
            <div class="textfield">{{ inputValue }}</div>
            <div class="icon">
                <micon icon="keyboard_arrow_down" />
            </div>
        </div>
        <div class="options" v-if="showingOptions">
            <div class="option empty" v-if="emptyOption !== false" @click="input(null)">{{ emptyOption }}</div>
            <div class="option" v-for="option in props.options" :key="option.value" @click="input(option.value)">{{ option.title }}</div>
        </div>
    </div>
</template>