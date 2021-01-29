<template>
  <section class="bg-white py-5 w-full">
    <div class="container mx-auto">
      <Logo class="w-32" />
      <div id="access" class="lg:flex justify-center my-10">
        <iframe
          class="map"
          src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3236.8206921969827!2d139.72080551526147!3d35.7797806801715!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x60189302bfcd40a1%3A0xec9bd41c22186537!2z44CSMTE1LTAwNDUg5p2x5Lqs6YO95YyX5Yy66LWk576977yS5LiB55uu77yR77yW4oiS77yR77yQ!5e0!3m2!1sja!2sjp!4v1611776212134!5m2!1sja!2sjp"
          frameborder="0"
          height="400px"
          style="border: 0"
          aria-hidden="false"
          :tabindex="0"
        />
        <div class="lg:w-1/2 text-left p-16">
          <div>住所：赤羽</div>
          <div>住所：赤羽</div>
          <div>住所：赤羽</div>
          <div>住所：赤羽</div>
          <div class="flex justify-center my-10 lg:px-20 gap-8">
            <a>
              <svg
                width="24"
                height="24"
                viewBox="0 0 48 48"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <rect width="48" height="48" fill="white" fill-opacity="0.01" />
                <path
                  d="M36 12.5997H31.2489H29.9871C28.9009 12.5997 28.0203 13.4803 28.0203 14.5666V21.4674H36L34.8313 29.0643H28.0203V43H19.2451V29.0643H12V21.4674H19.1515L19.2451 14.2563L19.2318 12.9471C19.1879 8.60218 22.6745 5.04434 27.0194 5.0004C27.0459 5.00013 27.0724 5 27.0989 5H36V12.5997Z"
                  fill="none"
                  stroke="#333"
                  stroke-width="4"
                  stroke-linejoin="round"
                />
              </svg>
            </a>
            <a>
              <svg
                width="24"
                height="24"
                viewBox="0 0 48 48"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <rect width="48" height="48" fill="white" fill-opacity="0.01" />
                <path
                  d="M34 6H14C9.58172 6 6 9.58172 6 14V34C6 38.4183 9.58172 42 14 42H34C38.4183 42 42 38.4183 42 34V14C42 9.58172 38.4183 6 34 6Z"
                  fill="none"
                  stroke="#333"
                  stroke-width="4"
                  stroke-linejoin="round"
                />
                <path
                  d="M24 32C28.4183 32 32 28.4183 32 24C32 19.5817 28.4183 16 24 16C19.5817 16 16 19.5817 16 24C16 28.4183 19.5817 32 24 32Z"
                  fill="none"
                  stroke="#333"
                  stroke-width="4"
                  stroke-linejoin="round"
                />
                <path
                  d="M35 15C36.1046 15 37 14.1046 37 13C37 11.8954 36.1046 11 35 11C33.8954 11 33 11.8954 33 13C33 14.1046 33.8954 15 35 15Z"
                  fill="#333"
                />
              </svg>
            </a>
            <a>
              <svg
                width="24"
                height="24"
                viewBox="0 0 48 48"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <rect width="48" height="48" fill="white" fill-opacity="0.01" />
                <path
                  d="M5 35.7622C6.92886 36.8286 20.8914 44.8773 30.8199 38.674C40.7483 32.4707 40.2006 21.7833 40.2006 16.886C41.1 15.0018 43 14.0439 43 8.9438C41.1337 10.6678 39.2787 11.2544 37.435 10.7036C35.6287 7.94957 33.1435 6.73147 29.9794 7.04934C25.2333 7.52614 23.4969 12.1825 24.0079 18.2067C16.6899 21.9074 10.9515 15.524 7.99418 10.7036C7.00607 14.4999 6.0533 19.0576 7.99418 24.0995C9.2881 27.4607 12.3985 30.3024 17.3254 32.6246C12.3323 35.3308 8.22382 36.3766 5 35.7622Z"
                  fill="none"
                  stroke="#333"
                  stroke-width="4"
                  stroke-linejoin="round"
                />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import Logo from "@/components/Logo.vue";

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