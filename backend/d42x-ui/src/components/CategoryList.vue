<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getCategoryList } from "../net/category";
import type { CategoryModel } from "../net/models";
import { RouterLink, useRoute } from "vue-router";
import { CategoryEmoji } from "../utilities/emoji";

const route = useRoute();
const categories = ref([] as CategoryModel[]);

const categoryQS = computed(() => {
  return route.query.category ?? "";
});

onMounted(async () => {
  categories.value = await getCategoryList();
});
</script>

<template>
  <div class="flex flex-col gap-4 h-full my-4 pr-2">
    <div class="flex items-center gap-2">
      <span class="text-xl">📔</span>
      <h2 class="text-xl font-semibold">热门分类</h2>
    </div>

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
      <li v-for="category in categories" :key="category.id">
        <RouterLink
          class="block w-auto px-2 py-1 rounded hover:bg-accent hover:text-accent-content"
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
        <div class="text-sm text-base-content/70 text-center py-4">多喝热水 🍵</div>
      </li>
    </ul>
  </div>
</template>
