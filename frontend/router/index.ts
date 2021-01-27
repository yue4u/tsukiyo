import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import { currentUser, waitForAuth } from "@/utils/auth";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: () => import("@/open/Home.vue"),
  },
  {
    path: "/admin",
    name: "Admin",
    component: () => import("@/admin/Console.vue"),
    // beforeEnter: (to, from, next) => {
    //   console.log(to.name);
    //   next();
    // },
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

router.beforeEach(async (to, _from, next) => {
  if (to.fullPath.startsWith("/admin") && to.name !== "Login") {
    const user = currentUser.value ?? (await waitForAuth());
    if (!user) {
      next({ name: "Login" });
      return;
    }
    next();
    return;
  }
  next();
});

export default router;
