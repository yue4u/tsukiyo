<template>
    <main>
        <Header />
        <h1 class="text-4xl my-10">Events</h1>
        <div v-if="fetching">Loading...</div>
        <div v-else-if="error">Oh no... {{ error }}</div>
        <div v-else>
            <ul v-if="data" class="grid gap-y-5 container mx-auto">
                <li v-for="event in data.events" :key="event.id" class="border-b-2">
                    <router-link
                        class="grid grid-cols-5 item"
                        :to="`/event/${event.slug ?? event.id}`"
                    >
                        <time class="text-gray-400">{{ dateTime.format(event.publishAt) }}</time>
                        <h2 class="text-bold text-left">{{ event.title }}</h2>
                        <span>
                            {{ event.pageView }}
                            views
                        </span>
                    </router-link>
                </li>
            </ul>
        </div>
    </main>
</template>

<script setup lang="ts">
import { useQuery } from "@urql/vue";
import type { Event } from "@/type/gql";
import { dateTime } from "@/utils";
import Header from '@/components/Header.vue'

const { fetching, data, error } = useQuery<{ events: Event[] }>({
    query: `
        {
          events {
            id
            slug
            title
            publishAt
            pageView
          }
        }
      `,
    requestPolicy: "network-only",
});
</script>

<style lang="scss" scoped>
.item {
    grid-template-columns: 10rem 1fr 10rem;
}
</style>
