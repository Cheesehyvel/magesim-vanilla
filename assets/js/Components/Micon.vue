<script setup>
const { icon } = defineProps(["icon"]);
</script>

<template>
    <span class="micon material-symbols-sharp">{{ icon }}</span>
</template>