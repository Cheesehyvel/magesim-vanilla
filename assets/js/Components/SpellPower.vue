<script setup>
import { computed } from "vue";

const { value } = defineProps(["value"]);

const lines = computed(() => {
    var arr = [];

    if (value.sp)
        arr.push({value: value.sp, type: "all", icon: null});
    if (value.sp_arcane)
        arr.push({value: value.sp_arcane, type: "arcane", icon: "ea25"});
    if (value.sp_fire)
        arr.push({value: value.sp_fire, type: "fire", icon: "ef55"});
    if (value.sp_frost)
        arr.push({value: value.sp_frost, type: "frost", icon: "eb3b"});

    return arr;
});
</script>

<template>
    <span class="spell-power">
        <template v-for="(line, index) in lines">
            <template v-if="index > 0"> / </template>
            <span :class="['sp-'+line.type]" :title="line.value+' '+line.type+' spell power'">
                {{ line.value }}
            </span>
        </template>
    </span>
</template>