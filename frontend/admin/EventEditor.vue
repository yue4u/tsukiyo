<template>
  <h1 v-if="fetching">fetching..</h1>
  <form
    v-else-if="isCreate || data?.event"
    class="form grid grid-cols-4 gap-2 text-lg"
    @submit.prevent="submit"
  >
    <Label class="w-10" for="title">title</Label>
    <input
      :class="inputClass"
      autocomplete="off"
      placeholder="title"
      id="title"
      v-model="form.event.title"
    />
    <Label class="w-10" for="slug">slug</Label>
    <input
      :class="inputClass"
      autocomplete="off"
      placeholder="/slug"
      id="slug"
      v-model="form.event.slug"
    />
    <Label class="w-10" for="genre">genre</Label>
    <input
      :class="inputClass"
      autocomplete="off"
      placeholder="genre"
      id="genre"
      v-model="form.event.genre"
    />
    <Label class="w-10" for="tag">tag</Label>
    <input
      :class="inputClass"
      autocomplete="off"
      placeholder="tag"
      id="tag"
      v-model="form.event.tag"
    />
    <Label class="w-10" for="fee">fee</Label>
    <input :class="inputClass" autocomplete="off" placeholder="0" id="fee" v-model="form.event.fee" />
    <Label class="w-10" for="ogpImg">ogpImg</Label>
    <input :class="inputClass" autocomplete="off" type="file" id="ogpImg" />
    <Label class="w-10" for="startAt">startAt</Label>
    <input
      :class="inputClass"
      autocomplete="off"
      id="startAt"
      v-model="form.event.startAt"
      type="date"
    />
    <Label class="w-10" for="endAt">endAt</Label>
    <input :class="inputClass" autocomplete="off" id="endAt" v-model="form.event.endAt" type="date" />
    <Label class="w-10" for="published">published</Label>
    <input
      :class="inputClass"
      class="w-min mx-5"
      type="checkbox"
      id="published"
      v-model="form.event.published"
    />
    <Label class="w-10 col-start-1" for="memo">memo</Label>
    <textarea
      class="memo col-span-3 h-36 border-b mx-5 focus:border-black outline-none resize-none"
      id="memo"
      placeholder="random memo"
      v-model="form.event.memo"
    />
    <textarea
      class="col-span-2 h-48 border-b mx-5 focus:border-black outline-none resize-none"
      v-model="form.event.body"
    />
    <div class="view col-span-2" v-html="markdown" />
    <div class="flex col-span-4 justify-between my-10 gap-2">
      <button
        class="w-full rounded-md bg-gray-100 hover:bg-gray-200 py-2"
        @click.prevent="returnToList"
      >return</button>

      <button
        class="w-full rounded-md bg-green-300 hover:bg-green-400 py-2"
        type="submit"
      >{{ isCreate ? "create" : "update" }}</button>
      <button
        v-if="!isCreate"
        class="border-2 w-full rounded-md border-red-200 text-red-400 hover:border-red-500 hover:bg-red-500 hover:text-white py-2"
        @click.prevent="deleteEvent()"
      >delete</button>
    </div>
  </form>
  <div class v-if="state.data">{{ state.data }}</div>
  <div class="text-red-500" v-if="state.error">{{ state.error }}</div>
</template>

<script setup lang="ts">
import { watch, reactive, computed, toRaw } from "vue";
import type { Event, EventInput, EventUpdate } from "@/type/gql";
import Label from "./EditorLabel.vue";
// @ts-ignore
import marked from "@/node_modules/marked/lib/marked.esm.js";
import { useQuery, useMutation } from "@urql/vue";
import { useRouter } from "vue-router";

const inputClass =
  "border-grey-300 border-b mx-5 focus:border-black outline-none";
const router = useRouter();
const { id } = router.currentRoute.value.query;

const isCreate = computed(() => !router.currentRoute.value.query.id);
const [now] = new Date().toISOString().split("T");

const returnToList = () => {
  router.push("/admin/event-list");
};

const form = reactive<{ event: EventInput }>({
  event: {
    slug: undefined,
    title: "",
    body: "# new event",
    genre: "genre",
    tag: undefined,
    fee: 0,
    ogpImg: undefined,
    startAt: now,
    endAt: now,
    published: false,
    memo: undefined,
  },
});

const { fetching, data } = useQuery<{ event: Event }>({
  query: `
    query ($id: Int!) {
      event(id: $id) {
         title
         body
         ogpImg
         startAt
         endAt
         publishAt
         updatedAt
         pageView
         createdAt
         published
       }
    }`,
  variables: { id: +id! },
  pause: isCreate,
  requestPolicy: "cache-and-network",
});

watch(
  () => data?.value,
  () => {
    if (data.value?.event) {
      form.event = { ...form.event, ...data.value.event };
    }
  }
);

const markdown = computed(() => {
  return marked(form.event.body, { sanitize: true });
});

const state = reactive<{
  data: any;
  error: string | undefined;
}>({
  data: undefined,
  error: undefined,
});

const submit = async () => {
  const action = async () => {
    const input: EventInput | EventUpdate = {
      ...toRaw(form.event),
      startAt: +form.event.startAt,
      endAt: +form.event.endAt,
    };
    if (isCreate.value) {
      return createEvent(input);
    } else {
      return updateEvent({ id: +id!, ...input });
    }
  };
  const { data, error } = await action();
  state.data = data;
  state.error = error?.message;
  if (error) {
    return;
  }
  returnToList();
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
    $startAt: NaiveDateTime,
    $endAt: NaiveDateTime,
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
          startAt: $startAt,
          endAt: $endAt,
          published: $published,
          memo: $memo
    }) {
      id
    }
  }
`);

const { executeMutation: updateEvent } = useMutation(`
  mutation (
    $id: Int!,
    $slug: String,
    $title: String!,
    $body: String!,
    $genre: String,
    $tag: String,
    $fee: Int,
    $ogpImg: String,
    $startAt: NaiveDateTime,
    $endAt: NaiveDateTime,
    $published: Boolean,
    $memo: String
  ){
      updateEvent(
        id: $id,
        update: {
          slug: $slug,
          title: $title,
          body: $body,
          genre: $genre,
          tag: $tag,
          fee: $fee,
          ogpImg: $ogpImg,
          startAt: $startAt,
          endAt: $endAt,
          published: $published,
          memo: $memo
    }) {
      id
    }
  }
`);

const { executeMutation: deleteEventMutation } = useMutation(`
  mutation ($id: Int!) {
    deleteEvent(id: $id) {
      id
    }
  }
`);

const deleteEvent = () => {
  deleteEventMutation({ id: +id! });
  returnToList();
};
</script>

<style lang="scss" scoped>
@import "../assets/base.scss";
.form {
  grid-template-columns: 8rem 1fr 8rem 1fr;
}
</style>
