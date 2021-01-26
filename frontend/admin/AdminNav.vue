<template>
  <nav>
    <ul>
      <li v-for="link in links" :key="link.path" class="py-4">
        <router-link class="block" :to="link.path">
          <span class="w-full block text-left font-bold">{{ link.title }}</span>
        </router-link>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
const links: { title: string; path: string }[] = [
  { title: "Contact list", path: "/admin/contact-list" },
  { title: "Event list", path: "/admin/event-list" },
  { title: "Create event", path: "/admin/event-editor" },
];
</script>
