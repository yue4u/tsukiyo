<template>
  <Logo />
  <Nav position="top" />
  <div class="container mx-auto my-8">
    <sub-title text="予約フォーム" />
    <div class="my-8">
      <form-item name="氏名：" :value="name" />
      <form-item name="メール：" :value="mail" />
      <form-item name="電話番号：" :value="tel" />
    </div>
    <div>
      <Selector name="月" :options="monthOptions" :value="month" />
      <Selector name="日" :options="monthOptions" :value="month" />
      <Selector name="時" :options="monthOptions" :value="month" />
      <Selector name="分" :options="monthOptions" :value="month" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import Logo from "@/components/design/Logo.vue";
// @ts-ignore
import Nav from "@/components/design/Nav.vue";
// @ts-ignore
import SubTitle from "@/components/design/SubTitle.vue";
// @ts-ignore
import FormItem from "@/components/design/FormItem.vue";
// @ts-ignore
import Selector from "@/components/design/Selector.vue";

const name = ref("");
const mail = ref("");
const tel = ref("");

const month = ref("");

const monthOptions = [
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "10",
  "11",
  "12",
];
</script>