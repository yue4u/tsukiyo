<template>
    <Logo class="w-24 my-5" />
    <nav class="bg-brown">
        <ul class="flex justify-around container mx-auto text-white mb-10 sm:text-base text-xs">
            <li v-for="item in menu" :key="item.text" class="my-8">
                <router-link :to="item.link">{{ item.text }}</router-link>
            </li>
        </ul>
    </nav>
</template>

<script setup lang="ts">
import Logo from './Logo.vue';
import { menu } from '@/router'

</script>