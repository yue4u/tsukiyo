<template>
  <main-visual />
  <div class="bg">
    <div class="container mx-auto">
      <Sentence />
      <Menu />
      <Event />
    </div>
  </div>
  <div class="container mx-auto">
    <Access />
  </div>
</template>

<script setup lang="ts">
import MainVisual from "./MainVisual.vue";
import Sentence from "./Sentence.vue";
import Menu from "./Menu.vue";
import Event from "./Event.vue";
import Access from "./Access.vue";
</script>

<style scoped>
.bg {
  background-color: var(--color-brown);
}
</style>