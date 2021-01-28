<template>
  <div class="h-screen w-full text-white">
    <img
      class="h-full w-full object-cover"
      src="/assets/DSC08501.jpeg"
      alt="MainVisual"
    />
    <div
      class="flex justify-center items-center h-full w-full absolute top-0 left-0"
    >
      <div class="w-24">
        <div class="text-8xl leading-tight">月夜のけだもの</div>
      </div>
    </div>
    <Nav position="right" />
  </div>
</template>

<script setup lang="ts">
import Nav from "./Nav.vue";
</script>
