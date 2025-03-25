<script setup lang="ts">
import { computed, onMounted, ref, useTemplateRef } from "vue";
import { getCategoryList } from "../net/category";
import type { CategoryModel } from "../net/models";
import { RouterLink, useRoute } from "vue-router";
import { CategoryEmoji } from "../utilities/emoji";

const SIZE = 25;
const page = ref(0);

const route = useRoute();
const searchEle = useTemplateRef("search-category");
const bottomBar = useTemplateRef("bottom-bar");

let observer: IntersectionObserver;

const searchText = ref("");
const categories = ref([] as CategoryModel[]);

const categoryQS = computed(() => {
  return route.query.category ?? "";
});

const showList = computed(() => {
  const pageStart = page.value > 0 ? page.value - 1 : 0;
  if (searchText.value === "") {
    return categories.value.slice(0, (pageStart + 1) * SIZE);
  }

  return categories.value.filter(
    (t) => t.name.indexOf(searchText.value) !== -1
  );
});

onMounted(async () => {
  page.value = 1;
  categories.value = await getCategoryList();

  observer = new IntersectionObserver((e) => {
    if (e.length <= 0) {
      return;
    }

    if (e[0].isIntersecting) {
      if (page.value * SIZE >= categories.value.length) {
        observer.disconnect();
      } else {
        page.value++;
      }
    }
  });
  observer.observe(bottomBar.value!);
});

function handleSearchCategory() {
  page.value = 1;
  const text = searchEle.value?.value?.trim() ?? "";
  searchText.value = text;
}

function handleSearchClose() {
  searchEle.value!.value = "";
  searchText.value = "";
  page.value = 1;
}
</script>

<template>
  <div class="flex flex-col gap-4 h-full my-4 pr-2">
    <div class="flex items-center gap-2">
      <span class="text-xl">📔</span>
      <h2 class="text-xl font-semibold">热门分类</h2>
    </div>

    <form @submit.prevent="handleSearchCategory">
      <label class="input input-xs w-auto ml-2">
        <span class="label mr-0">🔎</span>
        <input
          ref="search-category"
          type="search"
          placeholder="搜索标签"
          @keydown="handleSearchCategory"
        />
        <kbd class="kbd kbd-xs">回车</kbd>
        <kbd class="kbd kbd-xs cursor-pointer" @click="handleSearchClose"
          >X</kbd
        >
      </label>
    </form>

    <ul>
      <li>
        <RouterLink
          class="flex items-center gap-2 w-auto px-2 py-1 rounded hover:bg-accent hover:text-accent-content"
          to="/"
          :class="{
            'bg-neutral': !categoryQS,
            'text-neutral-content': !categoryQS,
          }"
        >
          <span class="text-xl">🗑️</span>
          全部</RouterLink
        >
      </li>
      <li v-for="category in showList" :key="category.id">
        <RouterLink
          class="block w-auto px-2 py-1 rounded hover:bg-accent hover:text-accent-content line-clamp-1 text-nowrap"
          :class="{
            'bg-info': categoryQS === category.name,
            'text-info-content': categoryQS === category.name,
          }"
          :to="`/?category=${category.name}`"
          >{{
            CategoryEmoji[Math.floor(Math.random() * CategoryEmoji.length)]
          }}
          {{ category.name }}</RouterLink
        >
      </li>
      <li>
        <div
          class="text-sm text-base-content/70 text-center py-4"
          ref="bottom-bar"
        >
          多喝热水 🍵
        </div>
      </li>
    </ul>
  </div>
</template>
