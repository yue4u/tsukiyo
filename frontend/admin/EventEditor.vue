<template>
  <h1 v-if="fetching">fetching..</h1>
  <form
    v-else-if="isCreate || data?.event"
    class="form grid grid-cols-4 gap-2 text-lg"
    @submit.prevent="submit"
  >
    <Label class="w-10" for="title">title</Label>
    <input
      id="title"
      v-model="form.event.title"
      :class="inputClass"
      autocomplete="off"
      placeholder="title"
    />
    <Label class="w-10" for="slug">slug</Label>
    <input
      id="slug"
      v-model="form.event.slug"
      :class="inputClass"
      autocomplete="off"
      placeholder="/slug"
    />
    <Label class="w-10" for="genre">genre</Label>
    <input
      id="genre"
      v-model="form.event.genre"
      :class="inputClass"
      autocomplete="off"
      placeholder="genre"
    />
    <Label class="w-10" for="tag">tag</Label>
    <input
      id="tag"
      v-model="form.event.tag"
      :class="inputClass"
      autocomplete="off"
      placeholder="tag"
    />
    <Label class="w-10" for="fee">fee</Label>
    <input id="fee" v-model="form.event.fee" :class="inputClass" autocomplete="off" placeholder="0" />
    <Label class="w-10 col-start-1" for="startAt">startAt</Label>
    <input
      id="startAt"
      v-model="form.event.startAt"
      :class="inputClass"
      autocomplete="off"
      type="date"
    />
    <Label class="w-10" for="endAt">endAt</Label>
    <input id="endAt" v-model="form.event.endAt" :class="inputClass" autocomplete="off" type="date" />
    <Label class="w-10" for="published">published</Label>
    <input
      id="published"
      v-model="form.event.published"
      :class="inputClass"
      class="w-min mx-5"
      type="checkbox"
    />

    <Label class="w-10 col-start-1" for="ogpImg">ogpImg</Label>
    <input
      id="ogpImg"
      :class="inputClass"
      autocomplete="off"
      type="file"
      :disabled="disbaled"
      @change="handleFileChange"
    />

    <img v-if="ogpURL" class="col-span-2" :src="ogpURL" />
    <Label class="w-10 col-start-1" for="memo">memo</Label>
    <textarea
      id="memo"
      v-model="form.event.memo"
      class="memo col-span-3 h-36 border-b mx-5 focus:border-black outline-none resize-none"
      placeholder="random memo"
    />
    <textarea
      v-model="form.event.body"
      class="col-span-2 h-48 border-b mx-5 focus:border-black outline-none resize-none"
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
  <div v-if="state.data" class>{{ state.data }}</div>
  <div v-if="state.error" class="text-red-500">{{ state.error }}</div>
</template>

<script setup lang="ts">
import { watch, ref, reactive, computed, toRaw } from "vue";
import type { Event, EventInput, EventUpdate } from "@/type/gql";
import Label from "./EditorLabel.vue";
// @ts-ignore
import marked from "@/node_modules/marked/lib/marked.esm.js";
import { useQuery, useMutation } from "@urql/vue";
import { useRouter } from "vue-router";
import { maybeTimestamp, maybeDateString } from '@/utils'
import { OGP } from '@/utils/stroage'

const { VITE_PUBLIC_BUCKET } = import.meta.env;

const inputClass =
  "border-grey-300 border-b mx-5 focus:border-black outline-none";
const router = useRouter();
const { id } = router.currentRoute.value.query;

const isCreate = computed(() => !router.currentRoute.value.query.id);
const [now] = new Date().toISOString().split("T");

const returnToList = () => {
  router.push("/admin/event-list");
};

type EventForm = {
  event: Omit<EventInput, 'startAt' | 'endAt'> & {
    startAt?: string | null, endAt?: string | null,
  }
}

const form = reactive<EventForm>({
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


const disbaled = ref(false);
const ogpLocal = ref<File | null>(null);
const ogpLocalURL = ref<string | null>(null);

watch(() => ogpLocal.value, async () => {
  if (!ogpLocal.value) return "";
  const arrayBuffer = await ogpLocal.value.arrayBuffer();
  const blob = new Blob([arrayBuffer]);
  ogpLocalURL.value = URL.createObjectURL(blob);
});

// TODO: fix this type
const handleFileChange = async (payload: any) => {
  if (!payload.target.files.length) return
  const file = (payload.target.files as FileList)[0];
  ogpLocal.value = file;
}


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
    if (!data.value?.event) {
      return
    }
    form.event = {
      ...form.event,
      ...data.value.event,
      startAt: maybeDateString(data.value.event.startAt),
      endAt: maybeDateString(data.value.event.endAt)
    };
  }
);

const ogpURL = computed(() => {
  if (form.event.ogpImg) {
    return `https://${VITE_PUBLIC_BUCKET}/ogp/${form.event.ogpImg}`
  }
  if (ogpLocalURL.value) {
    return ogpLocalURL.value
  }
  return undefined
})

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
    if (ogpLocal.value) {
      form.event.ogpImg = await OGP.upload(ogpLocal.value)
    }
    const input: EventInput | EventUpdate = {
      ...toRaw(form.event),
      startAt: maybeTimestamp(form.event.startAt),
      endAt: maybeTimestamp(form.event.endAt),
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
    if (form.event.ogpImg) {
      OGP.delete(form.event.ogpImg)
    }
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
    $title: String,
    $body: String,
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

const deleteEvent = async () => {
  deleteEventMutation({ id: +id! });
  if (form.event.ogpImg) {
    await OGP.delete(form.event.ogpImg);
  }
  returnToList();
};
</script>

<style lang="scss" scoped>
@import "../assets/base.scss";
.form {
  grid-template-columns: 8rem 1fr 8rem 1fr;
}
</style>
