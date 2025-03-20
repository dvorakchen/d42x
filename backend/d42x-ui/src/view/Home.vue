<script setup lang="ts">
import CategoryList from "../components/CategoryList.vue";
import MemeList from "../components/MemeList.vue";
import Return2Top from "../components/Return2Top.vue";
import Footer from "../components/Footer.vue";
import TopNav from "../components/TopNav.vue";
</script>

<template>
  <main
    class="relative grid grid-rows-[repeat(3,auto)] grid-cols-1 md:grid-cols-[repeat(3,auto)] min-h-screen"
  >
    <section
      class="col-start-1 col-end-4 sticky top-0 z-30 px-2 md:px-16 lg:px-8 py-2 bg-base-100"
    >
      <TopNav />
    </section>
    <section
      class="sticky top-16 justify-self-end hidden md:block w-48 h-[80vh] overflow-y-scroll"
    >
      <CategoryList />
    </section>

    <section class="lg:w-2xl px-4 py-8 lg:px-8 min-h-screen">
      <MemeList />
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
