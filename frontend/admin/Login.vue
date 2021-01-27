<template>
  <div>
    <h1 v-if="state.pending">waiting user...</h1>
    <form v-else-if="!state.user" class="p-10 mx-auto text-2xl" @submit.prevent="login">
      <label class="h-8 mr-10 font-bold text-right align-text-bottom" for="email" type="text">email</label>
      <input
        id="email"
        v-model="email"
        class="h-8 bg-transparent border-black border-b-2 focus:border-indigo-300"
        type="text"
      />
      <label
        class="h-8 mr-10 font-bold text-right align-text-bottom"
        for="password"
        type="text"
      >password</label>
      <input
        id="password"
        v-model="password"
        class="h-8 bg-transparent border-black border-b-2 focus:border-indigo-300"
        type="password"
      />
      <button
        class="box-border full w-full mt-20 m-auto bg-black text-white p-3 rounded-lg"
        type="submit"
      >login</button>
    </form>
    <p v-if="state.error" class="text-red-600 font-bold">{{ state.error }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useRouter } from "vue-router";
import { app } from "@/utils/auth";
import type { User } from "@/utils/auth";

const router = useRouter();
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
    router.push("/admin");
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
.label {
}
.error {
  color: red;
}
</style>
