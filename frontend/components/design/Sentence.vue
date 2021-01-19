<template>
  <div class="flex max-h-screen py-20 justify-between">
    <img class="w-3/5" src="/assets/DSC08504.jpeg" alt="" />
    <div class="w-2/5 text-white text-center">
      <div class="text-4xl my-10 inline-block under">宮沢賢治</div>
      <div class="mx-10">{{ text }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
export default {
  data() {
    return {
      text:
        "紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文紹介文",
    };
  },
};
</script>

<style scoped>
.under {
  line-height: 3em;
  border-bottom: 1px solid #fff;
}
</style>