<template>
    <h1>Contact</h1>
    <main class="mx-10">
        <form class="grid grid-cols-2 gap-2" @submit.prevent="handleSubmit">
            <label class="text-bold capitalize">title</label>
            <input v-model="form.title" :class="inputClass" type="text" />
            <label class="text-bold capitalize">name</label>
            <input v-model="form.name" :class="inputClass" type="text" />
            <label class="text-bold capitalize">email</label>
            <input v-model="form.email" :class="inputClass" type="text" />
            <label class="text-bold capitalize">phone</label>
            <input v-model="form.phone" :class="inputClass" type="text" />
            <label class="text-bold capitalize">body</label>
            <textarea v-model="form.body" :class="inputClass" />
            <button type="submit" class="text-white bg-black col-span-2">submit</button>
        </form>
        <pre v-if="state.error" class="text-left break-words whitespace-pre-wrap text-red-500">error...{{ state.error }}</pre>
        <pre v-else class="text-left break-words whitespace-pre-wrap">
        {{
            JSON.stringify(state.data, null, 4)
        }}
    </pre>
    </main>
</template>

<script setup lang="ts">
import { reactive, toRaw, } from 'vue'
import { useMutation, } from '@urql/vue'
import { inputClass } from '@/utils'
import type { Contact } from '@/type/gql'


const form = reactive(
    {
        title: "",
        name: "",
        email: "",
        phone: "",
        body: ""
    })


const state = reactive<{
    data: any;
    error: any;
}>({
    data: undefined,
    error: undefined,
});

const { executeMutation: createContact } = useMutation<{ createContact: Contact }>(`
      mutation (
        $title: String!
        $name: String!
        $email: String!
        $phone: String
        $body: String!
      ){
        createContact(contact: {
            title: $title, 
            name: $name, 
            email: $email, 
            phone: $phone
            body: $body
          }) {
            id
        }
      }`,
);


const handleSubmit = async () => {
    const { data, error } = await createContact(toRaw(form));
    state.data = JSON.stringify(data, null, 4);
    state.error = JSON.stringify(error, null, 4);
}
</script>
