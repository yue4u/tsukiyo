<template>
  <nav
    v-if="position == 'right'"
    class="absolute lg:text-xl top-0 right-0 h-full flex items-center lg:mr-10 mr-2"
  >
    <ul>
      <li v-for="item in menu" :key="item.text" class="my-8">
        <a :href="item.link">{{ item.text }}</a>
      </li>
    </ul>
  </nav>
  <nav v-if="position == 'top'" class="bg-gray-900 px-20">
    <ul class="flex justify-between container mx-auto text-white">
      <li v-for="item in menu" :key="item.text" class="my-8">
        <a :href="item.link">{{ item.text }}</a>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
const props = defineProps<{
  position: "top" | "right";
}>();
const menu = [
  { text: "About Us", link: "#about" },
  { text: "イベント一覧", link: "/event" },
  { text: "メニュー", link: "#menu" },
  { text: "アクセス", link: "#access" },
  { text: "お問い合わせ", link: "/contact" },
];
</script>
