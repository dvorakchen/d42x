<script setup lang="ts">
import { onMounted, ref } from "vue";
import CategoryList from "../components/CategoryList.vue";
import MemeList from "../components/MemeList.vue";
import Return2Top from "../components/Return2Top.vue";
import Footer from "../components/Footer.vue";
import Icon from "../components/Icon.vue";
import { mdiListBox } from "@mdi/js";

const slugs = ["呼唤爱与和平", "哈哈哈神经病"];

const showSlug = ref("");
const expanded = ref(false);

onMounted(() => {
  if (slugs.length > 0) {
    const index = Math.floor((Math.random() * 10) % slugs.length);
    showSlug.value = slugs[index];
  }
});

function handleExpand() {
  expanded.value = !expanded.value;
}
</script>

<template>
  <main class="min-h-screen flex flex-col">
    <div class="flex-grow w-full max-w-7xl m-auto pt-16 pb-16 px-4">
      <h1 class="text-4xl font-bold">
        <RouterLink to="/">🤣D42X</RouterLink>
        <span class="text-sm font-medium"> - {{ showSlug }}</span>
      </h1>
      <nav class="md:hidden pt-8 pb-4">
        <div class="flex flex-col">
          <button class="btn btn-square" @click="handleExpand">
            <Icon :d="mdiListBox" :size="30" />
          </button>
          <div
            class="grid transition-all duration-300"
            :class="{
              'grid-rows-[1fr]': expanded,
              'grid-rows-[0fr]': !expanded,
            }"
          >
            <div class="overflow-hidden">
              <CategoryList />
            </div>
          </div>
        </div>
      </nav>
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
  </main>
</template>
