<template>
  <main-visual />
  <div class="bg-yellow-900">
    <div class="container mx-auto">
      <Sentence />
    </div>
  </div>
</template>

<script lang="ts">
import MainVisual from "./MainVisual.vue";
import Sentence from "./Sentence.vue";

export default {
  components: {
    MainVisual,
    Sentence,
  },
};
</script>
