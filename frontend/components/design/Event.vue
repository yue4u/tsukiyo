<template>
  <div class="my-48 ml-32">
    <section-title text="イベント" />
  </div>
  <div class="text-box flex w-full p-10">
    <img class="w-2/5" src="/assets/DSC08492.jpeg" alt="" />
    <div class="w-full mx-16">
      <div class="text-center">Title</div>
      <div class="text-left">test test</div>
    </div>
  </div>
  <a href="#" class="bg-white inline-block my-10 px-16 py-6 text-2xl">
    View more >>
  </a>
</template>

<script setup lang="ts">
// @ts-ignore
import SectionTitle from "./SectionTitle.vue";
</script>

<style scoped>
.text-box {
  background-color: #fff;
}
.title {
  text-align: center;
}
</style>