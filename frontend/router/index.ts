import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../open/Home.vue";
import Order from "../open/Order.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/admin/login",
    name: "Login",
    component: () =>
      import(/* webpackChunkName: "Login" */ "@/admin/Login.vue"),
  },
  {
    path: "/admin/event-editor",
    name: "EventEditor",
    component: () =>
      import(/* webpackChunkName: "EventEditor" */ "@/admin/EventEditor.vue"),
  },
  {
    path: "/admin/event-list",
    name: "EventList",
    component: () =>
      import(/* webpackChunkName: "EventEditor" */ "@/admin/EventList.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
