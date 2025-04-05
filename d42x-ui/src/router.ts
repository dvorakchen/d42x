import { createRouter, createWebHistory } from "vue-router";
import Home from "./view/Home.vue";
import MemeList from "./components/MemeList.vue";

const routes = [
  {
    path: "/",
    component: Home,
    children: [
      {
        path: "",
        component: MemeList,
      },
      {
        path: "memes/:id",
        component: () => import("./components/MemeDetail.vue"),
      },
    ],
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior() {
    return { top: 0, behavior: "smooth" };
  },
});
