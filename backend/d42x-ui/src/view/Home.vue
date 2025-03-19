<script setup lang="ts">
import CategoryList from "../components/CategoryList.vue";
import MemeList from "../components/MemeList.vue";
import Return2Top from "../components/Return2Top.vue";
import Footer from "../components/Footer.vue";
import TopNav from "../components/TopNav.vue";
</script>

<template>
  <main
    class="grid grid-rows-[repeat(3,auto)] grid-cols-1 md:grid-cols-[repeat(3,auto)]"
  >
    <section
      class="col-start-1 col-end-4 sticky top-0 z-30 px-2 lg:px-16 py-2 bg-base-100"
    >
      <TopNav />
    </section>
    <section class="justify-self-end hidden md:block w-48">
      <CategoryList />
    </section>
    <section class="lg:w-xl p-8">
      <MemeList />
    </section>
    <section class="justify-self-start hidden lg:block w-48"></section>
    <section class="col-start-1 col-end-4">
      <Footer />
    </section>
  </main>
  <Return2Top />
  <!-- 
  <main class="min-h-screen flex flex-col">
    <div class="flex-grow w-full max-w-7xl m-auto pt-16 pb-16 px-4">
      <TopNav />
      <div class="pt-4 md:pt-16 flex gap-4">
        <section class="hidden md:block min-w-48 w-48">
          <CategoryList />
        </section>

        <section class="flex grow pl-8">
          <MemeList />
        </section>

        <section class="hidden lg:block min-w-48 w-48"></section>
      </div>
    </div>
    <Footer />
    <Return2Top />
  </main> -->
</template>
