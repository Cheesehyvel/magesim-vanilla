<script setup>
import icons from "../icons";
const { icon } = defineProps(["icon"]);
</script>

<template>
    <span class="wowicon"><img :src="icons.icon(icon)" :alt="icon"></span>
</template>