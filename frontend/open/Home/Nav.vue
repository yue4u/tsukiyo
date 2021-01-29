<template>
  <nav
    v-if="position == 'right'"
    class="absolute lg:text-xl top-0 right-0 h-full flex items-center lg:mr-10 mr-2"
  >
    <ul>
      <li v-for="item in menu" :key="item.text" class="my-8">
        <router-link :to="item.link">{{ item.text }}</router-link>
      </li>
    </ul>
  </nav>
  <nav v-if="position == 'top'" class="bg-gray-900 px-20">
    <ul class="flex justify-between container mx-auto text-white">
      <li v-for="item in menu" :key="item.text" class="my-8">
        <router-link :to="item.link">{{ item.text }}</router-link>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
import { menu } from '@/router'
const props = defineProps<{
  position: "top" | "right";
}>();
</script>
