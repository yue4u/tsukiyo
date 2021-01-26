<template>
  <div
    v-if="position == 'right'"
    class="absolute text-xl top-0 right-0 h-full flex items-center mr-10"
  >
    <ul>
      <li v-for="item in menu" :key="item.text" class="my-8">
        <a :href="item.link">{{ item.text }}</a>
      </li>
    </ul>
  </div>
  <div v-if="position == 'top'" class="bg-gray-900 px-20">
    <ul class="flex justify-between container mx-auto text-white">
      <li v-for="item in menu" :key="item.text" class="my-8">
        <a :href="item.link">{{ item.text }}</a>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  position: "top" | "right";
}>();
const menu = [
  { text: "About Us", link: "#" },
  { text: "Web予約", link: "#" },
  { text: "イベント一覧", link: "#" },
  { text: "メニュー", link: "#" },
  { text: "アクセス", link: "#" },
  { text: "お問い合わせ", link: "#" },
];
</script>
