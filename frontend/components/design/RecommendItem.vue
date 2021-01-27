<template>
  <div class="mx-10">
    <img :src="imgPath" alt="" />
    <div class="title text-white mt-6">{{ title }}</div>
    <div class="divider"></div>
    <div class="text-white mt-6">{{ text }}</div>
  </div>
</template>

<script setup="props" lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  imgPath: string;
  title: string;
  text: string;
}>();
</script>

<style scoped>
.divider {
  display: block;
  width: 80px;
  height: 4px;
  background-color: #7a6459;
  margin: 0 auto;
}
</style>