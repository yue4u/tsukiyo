<template>
  <div class="my-2">
    <input
      class="input px-1 py-2 outline-none"
      type="text"
      :placeholder="name"
      v-model="value"
    />
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  name: string;
  value: any;
}>();
</script>

<style scoped>
.input {
  border: 1px solid #000;
  min-width: 500px;
  color: #000;
}
</style>