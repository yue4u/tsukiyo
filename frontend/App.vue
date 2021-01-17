<template>
  <img alt="Vue logo" src="./assets/logo.png" />
  <EventList msg="Hello Vue 3 + Vite" />
  <EventEditor />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import EventList from "@/components/EventList.vue";
/**
 * NOTE: this is for demo, do not leave it here
 */
import EventEditor from "@/admin/EventEditor.vue";

export default defineComponent({
  name: "App",
  components: {
    EventList,
    EventEditor,
  },
});
</script>

<style>
html,
body {
  margin: 0;
}
html {
  font-size: 16px;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
li,
ul {
  list-style: none;
  margin: 0;
  padding: 0;
}
</style>