<template>
  <section class="h-screen w-full text-white">
    <img class="h-full w-full object-cover" src="/assets/DSC08501.jpeg" alt="MainVisual" />
    <div class="flex justify-center items-center h-full w-full absolute top-0 left-0">
      <h1 class="title w-24 lg:text-8xl text-6xl">月夜のけだもの</h1>
    </div>
    <Nav position="right" />
  </section>
</template>

<script setup lang="ts">
import Nav from "./Nav.vue";
</script>


<style scoped>
.title {
  line-height: 1.1;
  text-shadow: 0 0 10px #000;
}
</style>