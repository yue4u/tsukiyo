<template>
  <main class="bg">
    <MainVisual />
    <About />
    <Menu />
    <Event />
    <Access />
  </main>
</template>

<script setup lang="ts">
import MainVisual from "./MainVisual.vue";
import About from "./About.vue";
import Menu from "./Menu.vue";
import Event from "./Event.vue";
import Access from "./Access.vue";
</script>

<style scoped>
.bg {
  background-color: var(--color-brown);
}
</style>