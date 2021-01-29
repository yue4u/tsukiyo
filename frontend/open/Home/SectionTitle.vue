<template>
  <h2 class="title px-10 text-white text-4xl text-right mb-20 ml-5">{{ text }}</h2>
</template>

<script setup="props" lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  text: string;
}>();
</script>

<style scoped>
.title {
  width: 320px;
  height: 100px;
  border: 1px solid #fff;
  line-height: 100px;
  box-shadow: 10px 10px 0 #7a6459;
}
</style>