import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../open/Home.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/admin/event-editor",
    name: "About",
    component: () =>
      import(/* webpackChunkName: "EventEditor" */ "@/admin/EventEditor.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;