<template>
  <div class="main relative">
    <div class="content px-10 text-white text-4xl text-right">{{ text }}</div>
  </div>
</template>

<script setup="props" lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  text: string;
}>();
</script>

<style scoped>
.main {
  z-index: 1;
}
.content {
  width: 320px;
  height: 100px;
  border: 1px solid #fff;
  line-height: 100px;
  background-color: var(--color-brown);
}
.content::before {
  content: "";
  display: block;
  width: 320px;
  height: 100px;
  background: #7a6459;
  position: absolute;
  top: 10px;
  left: 10px;
  z-index: -1;
}
</style>