<template>
  <select name="blood">
    <!-- <option :value="null">{{ name }}</option> -->
    <option v-for="i in options" :key="i" :value="i">{{ i }}</option>
  </select>
</template>

<script setup="props" lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  name: string;
  options: string[];
  value: any;
}>();
</script>
