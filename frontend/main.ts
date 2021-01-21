import { createApp } from "vue";
import urql, {
  dedupExchange,
  cacheExchange,
  fetchExchange,
  errorExchange,
  ClientOptions,
} from "@urql/vue";
import App from "@/App.vue";
// import "@/registerServiceWorker";
import "./index.css";
import router from "@/router";
// TODO: conditionally import this since only admin needs this
import { authExchange } from "@/utils/auth";
const { VITE_API_ENDPOINT } = import.meta.env;

// import {produce} from 'immer'
/**
 * @see https://formidable.com/open-source/urql/docs/advanced/authentication/
 */
const urqlOptions: ClientOptions = {
  url: VITE_API_ENDPOINT as string,
  exchanges: [
    dedupExchange,
    cacheExchange,
    errorExchange({ onError() {} }),
    authExchange,
    fetchExchange,
  ],
};

createApp(App).use(router).use(urql, urqlOptions).mount("#app");
