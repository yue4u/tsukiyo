<template>
  <div class="absolute text-xl top-0 right-0 h-full flex items-center mr-10">
    <ul>
      <li v-for="item in menu" :key="item.text" class="my-8">
        <a :href="item.link">{{ item.text }}</a>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
export default {
  data() {
    return {
      menu: [
        { text: "About Us", link: "#" },
        { text: "Web予約", link: "#" },
        { text: "イベント一覧", link: "#" },
        { text: "メニュー", link: "#" },
        { text: "アクセス", link: "#" },
        { text: "問合せ", link: "#" },
      ],
    };
  },
};
</script>