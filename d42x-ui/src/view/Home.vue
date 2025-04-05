<script setup lang="ts">
import CategoryList from "../components/CategoryList.vue";
import Return2Top from "../components/Return2Top.vue";
import Footer from "../components/Footer.vue";
import TopNav from "../components/TopNav.vue";
import Icon from "../components/Icon.vue";
import { mdiArrowLeft } from "@mdi/js";
import { useRoute, useRouter } from "vue-router";
import { computed } from "vue";

const router = useRouter();
const route = useRoute();

const backAllow = computed(() => {
  return route.path !== "/";
});

function handleRouteBack() {
  router.back();
}
</script>

<template>
  <main
    class="relative grid grid-rows-[repeat(3,auto)] grid-cols-1 md:grid-cols-[repeat(3,auto)] min-h-screen"
  >
    <section
      class="col-start-1 col-end-4 sticky top-0 z-30 px-2 md:px-8 py-2 bg-base-100"
    >
      <TopNav />
    </section>
    <section
      class="sticky top-16 justify-self-end hidden md:block w-48 h-[80vh] overflow-y-scroll"
    >
      <CategoryList />
    </section>

    <section
      class="flex flex-col lg:w-2xl px-4 pt-2 md:pt-8 py-8 lg:px-8 min-h-screen"
    >
      <div class="py-2">
        <button
          class="btn btn-md btn-circle btn-ghost"
          @click="handleRouteBack"
          :disabled="!backAllow"
        >
          <Icon :d="mdiArrowLeft" />
        </button>
      </div>
      <RouterView />
    </section>
    <section class="justify-self-start hidden lg:block w-48"></section>
    <section class="static z-10 col-start-1 col-end-4">
      <div class="flex flex-col justify-end h-full">
        <Footer />
      </div>
    </section>
  </main>
  <Return2Top />
</template>
