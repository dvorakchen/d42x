import { createRouter, createWebHistory } from "vue-router";
import Home from "./view/Home.vue";

const routes = [
  {
    path: "/",
    component: Home,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior() {
    return { top: 0, behavior: "smooth" };
  },
});
