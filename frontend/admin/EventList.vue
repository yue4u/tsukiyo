<template>
  <div v-if="fetching">Loading...</div>
  <div v-else-if="error">Oh no... {{ error }}</div>
  <div v-else>
    <div v-if="!data?.events?.length">There is no events</div>
    <p v-else class="text-left text-lg mb-5">There is {{ data.events.length }} events</p>

    <ul v-if="data" class="grid gap-y-5">
      <li v-for="event in data.events" :key="event.id">
        <router-link class="grid grid-cols-5" :to="`/admin/event-editor?id=${event.id}`">
          <span
            v-if="event.published"
            class="inline-grid place-items-center w-18 px-3 bg-green-200 rounded-md"
          >published</span>
          <span v-else class="inline-grid place-items-center w-18 px-3 bg-gray-200 rounded-md">draft</span>
          <span>{{ event.title }}</span>
          <span>
            {{ event.pageView }}
            views
          </span>
          <time>{{ dateTime.format(event.createdAt) }}</time>
          <time>{{ dateTime.format(event.updatedAt) }}</time>
        </router-link>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useQuery, useMutation } from "@urql/vue";
import type { Event } from "@/type/gql";
import { dateTime } from "@/utils";

const { fetching, data, error } = useQuery<{ events: Event[] }>({
  query: `
        {
          events {
            id
            title
            body
            ogpImg
            startAt
            endAt
            publishAt
            updatedAt
            pageView
            creatorId
            createdAt
            published
          }
        }
      `,
  requestPolicy: "network-only",
});
</script>

<style lang="scss" scoped></style>
