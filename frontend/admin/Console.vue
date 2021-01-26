<template>
  <header class="bg-black mb-5">
    <Logo class="-translate-y-10 shadow-lg" />
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
import { provide } from "vue";
import { useRoute } from "vue-router";
import Logo from "@/components/Logo.vue";
// @ts-ignore
import AdminNav from "@/admin/AdminNav.vue";
import { events, eventsSymbol, contacts, contactsSymbol } from './admin-store'
const isLogin = useRoute().name == "Login";

provide(eventsSymbol, events);
provide(contactsSymbol, contacts);

</script>

<style lang="scss" scoped>
.grid {
  grid-template-columns: auto 1fr;
}
</style>