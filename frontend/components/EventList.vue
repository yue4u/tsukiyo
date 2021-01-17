<template>
  <div v-if="!data?.events?.length">There is no events</div>
  <div v-else>There is {{ data.events.length }} events</div>
  <div v-if="fetching">Loading...</div>
  <div v-else-if="error">Oh no... {{ error }}</div>
  <div v-else>
    <ul v-if="data">
      <li
        class="event"
        v-for="event in data.events"
        :key="event.id"
        @click="deleteEvent({ id: event.id })"
      >
        {{ event.id }} )
        {{ event.title }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useQuery, useMutation } from "@urql/vue";
import { onMounted } from "vue";

onMounted(() => {
  executeQuery({
    requestPolicy: "network-only",
  });
});

const { fetching, data, error, executeQuery } = useQuery({
  query: `
        {
          events {
            id
            title
          }
        }
      `,
});

const { executeMutation: deleteEvent } = useMutation(`
  mutation ($id: Int!) {
    deleteEvent(id: $id) {
      id
    }
  }
`);
</script>

<style lang="scss" scoped>
.event {
  font-size: 2rem;
  cursor: pointer;
  &:hover {
    &::after {
      margin-left: 1rem;
      content: "x";
      color: red;
    }
  }
}
</style>
