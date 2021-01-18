import { createApp } from "vue";
import urql from "@urql/vue";
import App from "./App.vue";
import "./registerServiceWorker";
import router from "./router";
import './index.css'

createApp(App)
  .use(router)
  .use(urql, {
    url: "http://localhost:4000/graphql",
  })
  .mount("#app");
