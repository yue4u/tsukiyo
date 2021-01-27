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
// @ts-ignore
import MainVisual from "./MainVisual.vue";
// @ts-ignore
import Sentence from "./Sentence.vue";
// @ts-ignore
import Menu from "./Menu.vue";
// @ts-ignore
import Event from "./Event.vue";
// @ts-ignore
import Access from "./Access.vue";
</script>

<style scoped>
.bg {
  background-color: #23140a;
}
</style>