<template>
  <section class="container mx-auto my-32">
    <SectionTitle text="イベント" />
    <div class="lg:flex block w-full p-10 text-white box">
      <img class="lg:w-2/5 w-full" src="/assets/DSC08492.jpeg" />
      <div class="w-full lg:mx-16 lg:my-0 my-10">
        <div class="text-center">Title</div>
        <div class="text-left">test test</div>
      </div>
    </div>
    <router-link to="/event" class="inline-block my-10 px-16 py-6 text-2xl bg-wheat">View more >></router-link>
  </section>
</template>

<script setup lang="ts">
import SectionTitle from "./SectionTitle.vue";
</script>

<style scoped>
.title {
  text-align: center;
}
.box {
  border: 1px wheat solid;
}
</style>