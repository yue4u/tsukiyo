import { createApp } from "vue";
import urql from "@urql/vue";
import App from "./App.vue";

createApp(App)
  .use(urql, {
    url: "http://localhost:4000/graphql",
  })
  .mount("#app");
