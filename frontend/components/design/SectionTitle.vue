<template>
  <div>
    <div>
      {{ text }}
    </div>
  </div>
</template>

<script setup="props" lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  text: string;
}>();
</script>
