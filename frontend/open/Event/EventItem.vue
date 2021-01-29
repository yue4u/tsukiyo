<template>
    <main>
        <Header />
        <p v-if="fetching">fetching...</p>
        <p v-else-if="error">error...{{ error }}</p>
        <article v-else class="container mx-auto">
            <h1 class="text-4xl my-10">{{ data?.event.title }}</h1>
            <p>{{ data?.event.pageView }}views</p>
            <time class="block my-5">{{ dateTime.format(data?.event.publishAt) }}</time>
            <img v-if="imgSrc" class="my-10 mx-auto" :src="imgSrc" />
            <div v-html="markdown"></div>
        </article>
    </main>
</template>

<script setup lang="ts">
import { computed, toRaw } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useQuery } from '@urql/vue'
import marked from "@/node_modules/marked/lib/marked.esm.js";
import Header from '@/components/Header.vue'
import type { Event } from '@/type/gql'
import { dateTime } from '@/utils'

const router = useRouter();
const { id_or_slug } = useRoute().params;

const query = computed(() => {
    if (Array.isArray(id_or_slug)) { router.push('/'); return }
    let maybeInt = parseInt(id_or_slug, 10);
    if (maybeInt.toString() === id_or_slug) {
        return {
            id: maybeInt,
        }
    } else {
        return {
            slug: id_or_slug,
        }
    }
})

const { fetching, data, error } = useQuery<{ event: Event }>({
    query: `
    query ($slug: String, $id: Int) {
      event(query: {slug: $slug, id: $id}) {
         id
         title
         body
         ogpImg
         startAt
         endAt
         publishAt
         pageView
       }
    }`,
    variables: { ...query.value },
    requestPolicy: "cache-and-network",
});

const markdown = computed(() => {
    return data.value?.event ? marked(data.value?.event.body, { sanitize: true }) : "";
});

const imgSrc = computed(() => {
    if (!data.value?.event.ogpImg) return null
    return `https://${import.meta.env.VITE_PUBLIC_BUCKET_NAME}/ogp/${data.value?.event.ogpImg}`
})
</script>
