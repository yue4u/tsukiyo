import { createApp } from "vue";
import urql, {
  dedupExchange,
  cacheExchange,
  fetchExchange,
  errorExchange,
  ClientOptions,
} from "@urql/vue";
import App from "@/App.vue";
import "@/registerServiceWorker";
import router from "@/router";
// TODO: conditionally import this since only admin needs this
import { authExchange } from "@/utils/auth";
// import {produce} from 'immer'
/**
 * @see https://formidable.com/open-source/urql/docs/advanced/authentication/
 */
const urqlOptions: ClientOptions = {
  url: "http://localhost:4000/graphql",
  exchanges: [
    dedupExchange,
    cacheExchange,
    errorExchange({ onError() {} }),
    authExchange,
    fetchExchange,
  ],
};

createApp(App).use(router).use(urql, urqlOptions).mount("#app");
