<template>
    <Header />
    <main class="mx-10">
        <h1 class="text-4xl my-10">Contact</h1>
        <form
            class="grid grid-cols-2 gap-2 form container mx-auto text-2xl text-left"
            @submit.prevent="handleSubmit"
        >
            <label class="text-bold capitalize">title</label>
            <input v-model="form.title" :class="inputClass" type="text" />
            <label class="text-bold capitalize">name</label>
            <input v-model="form.name" :class="inputClass" type="text" />
            <label class="text-bold capitalize">email</label>
            <input v-model="form.email" :class="inputClass" type="text" />
            <label class="text-bold capitalize">phone</label>
            <input v-model="form.phone" :class="inputClass" type="text" />
            <label class="text-bold capitalize">body</label>
            <textarea v-model="form.body" :class="[inputClass, 'h-48']" />
            <div class="col-span-2 grid place-content-center">
                <button
                    type="submit"
                    class="text-white bg-black col-span-2 w-48 px-5 py-5 rounded-xl text-2xl mt-10"
                >submit</button>
            </div>
        </form>
        <pre v-if="state.error" class="text-left break-words whitespace-pre-wrap text-red-500">error...{{ state.error }}</pre>
    </main>
</template>

<script setup lang="ts">
import { reactive, toRaw, } from 'vue'
import { useMutation, } from '@urql/vue'
import Header from '@/components/Header.vue'
import { inputClass } from '@/utils'
import type { Contact } from '@/type/gql'
import router from '@/router'


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
    if (!error) {
        router.push('/contact/success');
    }
    state.data = JSON.stringify(data, null, 4);
    state.error = error?.toString()
}
</script>

<style>
.form {
    grid-template-columns: 10rem 1fr;
}
</style>