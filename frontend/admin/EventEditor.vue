<template>
  <h1>create new event</h1>
  <form class="form" @submit.prevent="submit">
    <label for="title">title</label>
    <input autocomplete="off" id="title" v-model="title" />
    <label for="slug">slug</label>
    <input autocomplete="off" id="slug" v-model="slug" />
    <label for="genre">genre</label>
    <input autocomplete="off" id="genre" v-model="genre" />
    <label for="tag">tag</label>
    <input autocomplete="off" id="tag" v-model="tag" />
    <label for="fee">fee</label>
    <input autocomplete="off" id="fee" v-model="fee" />
    <label for="ogpImg">ogpImg</label>
    <input autocomplete="off" id="ogpImg" v-model="ogpImg" />
    <label for="startTime">startTime</label>
    <input autocomplete="off" id="startTime" v-model="startTime" type="date" />
    <label for="endTime">endTime</label>
    <input autocomplete="off" id="endTime" v-model="endTime" type="date" />
    <label for="published">published</label>
    <input
      autocomplete="off"
      type="checkbox"
      id="published"
      v-model="published"
    />
    <label for="memo">memo</label>
    <input autocomplete="off" id="memo" v-model="memo" />
    <div class="view" v-html="markdown" />
    <textarea v-model="body" />
    <button class="create full" type="submit">create</button>
  </form>
  <div class="info" v-if="state.data">{{ state.data }}</div>
  <div class="info error" v-if="state.error">{{ state.error }}</div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from "vue";
// @ts-ignore
import marked from "@/node_modules/marked/lib/marked.esm.js";
import { useMutation, CombinedError } from "@urql/vue";

const [now] = new Date().toISOString().split("T");
const slug = ref("");
const title = ref("");
const body = ref("# new event");
const genre = ref("genre");
const tag = ref("");
const fee = ref(0);
const ogpImg = ref("");
const startTime = ref(now);
const endTime = ref(now);
const published = ref(false);
const memo = ref("");

const markdown = computed(() => {
  return marked(body.value, { sanitize: true });
});

const state = reactive({
  data: undefined,
  error: undefined as string | undefined,
});

const submit = async () => {
  const { data, error } = await createEvent({
    slug: slug.value,
    title: title.value,
    body: body.value,
    genre: genre.value,
    tag: tag.value,
    fee: fee.value,
    ogpImg: ogpImg.value,
    startTime: +new Date(startTime.value),
    endTime: +new Date(endTime.value),
    published: published.value,
    memo: memo.value,
  });
  state.data = data;
  state.error = error?.message;
};

const { executeMutation: createEvent } = useMutation(`
  mutation (
    $slug: String, 
    $title: String!, 
    $body: String!, 
    $genre: String, 
    $tag: String, 
    $fee: Int, 
    $ogpImg: String, 
    $startTime: NaiveDateTime, 
    $endTime: NaiveDateTime, 
    $published: Boolean, 
    $memo: String
  ){
      createEvent(
        event: {
          slug: $slug, 
          title: $title, 
          body: $body, 
          genre: $genre, 
          tag: $tag, 
          fee: $fee, 
          ogpImg: $ogpImg, 
          startTime: $startTime, 
          endTime: $endTime, 
          published: $published, 
          memo: $memo
    }) {
      id
    }
  }
`);
</script>

<style lang="scss" scoped>
@import "../assets/base.scss";

.form {
  margin: 0 auto;
  max-width: 1000px;
  display: grid;
  gap: 1rem;
  grid-template-columns: 1fr 1fr;
}
.create {
  outline: none;
  appearance: none;
  border: 0;
  padding: 1rem;
  cursor: pointer;
}

label {
  font-weight: bold;
}
input,
label {
  place-content: center;
  padding: 5px;
  font-size: 1rem;
}

input {
  border: 0;
  outline: none;
  appearance: none;
  border-bottom: 1px solid #ccc;
}
.info {
  margin: 1rem;
}
.error {
  color: red;
}
.view {
  word-break: break-all;
}
textarea {
  border: 0;
  background: #eee;
  padding: 5px;
}
</style>
