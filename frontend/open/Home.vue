<template>
  <Main />
</template>

<script lang="ts">
// this is caused by <script setup>, looks vetur does not support it for now
// @ts-ignore
import EventList from "@/components/EventList.vue";
import Main from "@/components/design/Main.vue";

export default {
  name: "home",
  components: {
    EventList,
    Main,
  },
};
</script>

<style>
/*
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
}*/
</style>
