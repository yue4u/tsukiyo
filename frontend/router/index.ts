import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../open/Home.vue";
import Order from "../open/Order.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: import("@/open/Home.vue"),
  },
  {
    path: "/admin",
    name: "Admin",
    component: () => import("@/admin/Console.vue"),
    children: [
      {
        path: "/admin/login",
        name: "Login",
        component: () => import("@/admin/Login.vue"),
      },
      {
        path: "/admin/contact-list",
        name: "ContactList",
        component: () => import("@/admin/ContactList.vue"),
      },
      {
        path: "/admin/event-editor",
        name: "EventEditor",
        component: () => import("@/admin/EventEditor.vue"),
      },
      {
        path: "/admin/event-list",
        name: "EventList",
        component: () => import("@/admin/EventList.vue"),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
