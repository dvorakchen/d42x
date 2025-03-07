<script setup lang="ts">
import { onMounted, ref } from "vue";
import CategoryList from "../components/CategoryList.vue";
import MemeList from "../components/MemeList.vue";
import Return2Top from "../components/Return2Top.vue";

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
  <div class="w-full max-w-5xl m-auto pt-16 pb-16 px-4">
    <h1 class="text-4xl font-bold">
      <a href="/">D42X</a>
      <span class="text-sm font-medium"> - {{ showSlug }}</span>
    </h1>
    <nav class="md:hidden pt-8">
      <div class="flex flex-col">
        <div>
          <button class="btn btn-circle" @click="handleExpand">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              fill="currentColor"
              class="bi bi-list"
              viewBox="0 0 16 16"
            >
              <path
                fill-rule="evenodd"
                d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5"
              />
            </svg>
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
      </div>
    </nav>
    <div class="pt-4 md:pt-16 flex gap-4">
      <section class="hidden md:block min-w-48 w-48">
        <CategoryList />
      </section>

      <section class="flex grow">
        <MemeList />
      </section>

      <section class="hidden lg:block min-w-48 w-48"></section>
    </div>
  </div>
  <footer
    class="footer sm:footer-horizontal footer-center bg-base-300 text-base-content p-4"
  >
    <aside>
      <p>
        Copyright © {{ new Date().getFullYear() }} - All right reserved by D42X
        Dvorak
      </p>
      <p>
        <a href="https://beian.miit.gov.cn/" target="_blank"
          >粤ICP备2024191084号-2</a
        >
      </p>
    </aside>
  </footer>
  <Return2Top />
</template>
