<template>
  <div v-if="fetching">Loading...</div>
  <div v-else-if="error">Oh no... {{ error }}</div>
  <div v-else>
    <div v-if="!data?.events?.length">There is no events</div>
    <p class="text-left" v-else>There is {{ data.events.length }} events</p>

    <ul class="grid gap-y-5" v-if="data">
      <li v-for="event in data.events" :key="event.id">
        <router-link class="grid grid-cols-5" :to="`/admin/event-editor?id=${event.id}`">
          <span
            class="inline-grid place-items-center w-min px-3 bg-green-300"
            v-if="event.published"
          >published</span>
          <span class="inline-grid place-items-center w-min px-3 bg-gray-300" v-else>draft</span>
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
import { watch } from "vue";
import type { Event } from "@/type/gql";
import { dateTime } from '@/utils'
import { useEvents } from './admin-store'


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
      `, requestPolicy: 'network-only'
});

const events = useEvents();

watch(() => data, () => {
  data?.events.forEach(event => {
    events[event.id] = event
  });
})

const { executeMutation: deleteEvent } = useMutation(`
  mutation ($id: Int!) {
    deleteEvent(id: $id) {
      id
    }
  }
`);
</script>

<style lang="scss" scoped>
</style>
