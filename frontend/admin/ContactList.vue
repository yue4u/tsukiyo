<template>
  <div v-if="!data?.contacts?.length">There is no contacts</div>
  <AdminViewTitle v-else>There is {{ data.contacts.length }} contacts</AdminViewTitle>
  <div v-if="fetching">Loading...</div>
  <div v-else-if="error">Oh no... {{ error }}</div>
  <div v-else>
    <ul class="grid grid-cols-8 row">
      <span>id</span>
      <span>checked</span>
      <span>title</span>
      <span>name</span>
      <span>email</span>
      <span>phone</span>
      <span>body</span>
      <span>createdAt</span>
    </ul>
    <ul v-if="data">
      <li
        v-for="contact in data.contacts"
        :key="contact.id"
        class="grid grid-cols-8 border-b-2 py-2 row"
      >
        <span>#{{ contact.id }}</span>
        <span>{{ contact.checked }}</span>
        <span>{{ contact.title }}</span>
        <span>{{ contact.name }}</span>
        <span>{{ contact.email }}</span>
        <span>{{ contact.phone }}</span>
        <span>{{ contact.body }}</span>
        <span class="relative">
          {{ dateTime.format(contact.createdAt) }}
          <button
            class="right-0 top-0 absolute bg-red-500 text-white rounded-lg w-full h-full grid opacity-0 hover:opacity-100"
            @click="deleteContact({ id: contact.id })"
          >delete</button>
        </span>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useQuery, useMutation } from "@urql/vue";
import type { Contact } from '@/type/gql'
import { dateTime } from '@/utils'
import AdminViewTitle from './AdminViewTitle.vue'

const { fetching, data, error } = useQuery<{ contacts: Contact[] }>({
  query: `
      {
        contacts{
          id
          title
          name
          email
          phone
          body
          createdAt
          checked
        }
      }
      `,
  requestPolicy: "network-only",
});

const { executeMutation: deleteContact } = useMutation(`
  mutation ($id: Int!) {
    deleteContact(id: $id) {
      id
    }
  }
`);
</script>

<style lang="scss" scoped>
.row {
  grid-template-columns: 1rem 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
}
</style>
