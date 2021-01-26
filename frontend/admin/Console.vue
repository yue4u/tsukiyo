<template>
  <header class="bg-black mb-5 relative h-20">
    <Logo class="shadow-lg" />
    <span
      v-if="currentUser"
      class="text-white inline-grid absolute right-2 top-0 z-10 h-20 place-items-center justify-center"
    >
      {{ currentUser?.email }}
      <span class="underline cursor-pointer" @click="logout">Logout</span>
    </span>
  </header>
  <div v-if="isLogin">
    <router-view />
  </div>
  <div v-else class="grid grid-cols-2">
    <aside class="p-4">
      <AdminNav />
    </aside>
    <main class="p-4">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import Logo from "@/components/Logo.vue";

// @ts-ignore
import AdminNav from "@/admin/AdminNav.vue";
import { currentUser, logout } from "@/utils/auth";

const isLogin = useRoute().name == "Login";
</script>

<style lang="scss" scoped>
.grid {
  grid-template-columns: auto 1fr;
}
</style>
