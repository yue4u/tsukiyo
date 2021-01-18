<template>
  <h1 v-if="state.pending">waiting user...</h1>
  <form v-else-if="!state.user" @submit.prevent="login">
    <label for="email" type="text">email</label>
    <input id="email" type="text" v-model="email" />
    <label for="password" type="text">password</label>
    <input id="password" type="text" v-model="password" />
    <button class="full" type="submit">login</button>
  </form>
  <p v-if="state.user">{{ state.user }}</p>
  <p class="error" v-if="state.error">{{ state.error }}</p>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { app } from "./auth";
import type { User } from "./auth";

const email = ref("");
const password = ref("");
const state = reactive({
  pending: true,
  user: null as User | null,
  error: null as string | null,
});

app.auth().onAuthStateChanged((maybeUser) => {
  state.pending = false;
  if (maybeUser) {
    state.user = maybeUser;
    state.error = null;
  } else {
    state.user = null;
  }
});

const login = async () => {
  app
    .auth()
    .signInWithEmailAndPassword(email.value, password.value)
    .catch((e) => {
      state.error = e.message;
    });
};
</script>

<style lang="scss" scoped>
@import "../assets/base.scss";

form {
  display: grid;
  grid-template-columns: 1fr 1fr;
}
.error {
  color: red;
}
</style>
