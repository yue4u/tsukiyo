<template>
  <Logo />
  <div id="access" class="lg:flex justify-center my-10">
    <iframe
      class="map"
      src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3236.8206921969827!2d139.72080551526147!3d35.7797806801715!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x60189302bfcd40a1%3A0xec9bd41c22186537!2z44CSMTE1LTAwNDUg5p2x5Lqs6YO95YyX5Yy66LWk576977yS5LiB55uu77yR77yW4oiS77yR77yQ!5e0!3m2!1sja!2sjp!4v1611776212134!5m2!1sja!2sjp"
      frameborder="0"
      height="400px"
      style="border: 0"
      aria-hidden="false"
      :tabindex="0"
    ></iframe>
    <div class="lg:w-1/2 text-left p-16">
      <div>住所：赤羽</div>
      <div>住所：赤羽</div>
      <div>住所：赤羽</div>
      <div>住所：赤羽</div>
      <div class="w-full flex justify-between my-10 lg:px-20">
        <a>Facebook Icon</a>
        <a>Instgram Icon</a>
        <a>Twitter Icon</a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Logo from "./Logo.vue";
</script>

<style scoped>
.map {
  width: 50%;
}
@media (max-width: 1024px) {
  .map {
    width: 100%;
  }
}
</style>