<template>
  <div>
    <div class="text py-2 px-2 inline-block text-2xl">{{ text }}</div>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  text: string;
}>();
</script>

<style scoped>
.text {
  border-bottom: 2px solid #888;
}
</style>