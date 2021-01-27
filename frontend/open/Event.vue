<template>
    <h1>{{ $route.params.id_or_slug }}</h1>
    <p v-if="fetching">fetching...</p>
    <p v-else-if="error">error...{{ error }}</p>
    <pre v-else>
        {{
            JSON.stringify(data, null, 4)
        }}
    </pre>
</template>

<script setup lang="ts">
import { computed, toRaw } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useQuery } from '@urql/vue'

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
         updatedAt
         pageView
         createdAt
         published
       }
    }`,
    variables: { ...query.value },
    requestPolicy: "cache-and-network",
});
</script>
