<template>
  <div v-if="!data?.contacts?.length">There is no contacts</div>
  <div v-else>There is {{ data.contacts.length }} contacts</div>
  <div v-if="fetching">Loading...</div>
  <div v-else-if="error">Oh no... {{ error }}</div>
  <div v-else>
    <ul v-if="data">
      <li v-for="contact in data.contacts" :key="contact.id" class="contact">
        {{ contact.id }} )
        {{ contact.title }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useQuery } from "@urql/vue";

const { fetching, data, error } = useQuery({
  query: `
        {
          contacts {
            id
            name
            title
          }
        }
      `,
  requestPolicy: "network-only",
});
</script>

<style lang="scss" scoped>
.contact {
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
