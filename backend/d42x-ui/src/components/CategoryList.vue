<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getCategoryList } from "../net/category";
import type { CategoryModel } from "../net/models";
import Icon from "./Icon.vue";
import { mdiDotsGrid, mdiShapePlus } from "@mdi/js";
import { RouterLink } from "vue-router";

const categories = ref([] as CategoryModel[]);

onMounted(async () => {
  categories.value = await getCategoryList();
});
</script>

<template>
  <div
    class="flex flex-col gap-4 h-full py-4 pr-2 border-r border-base-content"
  >
    <div class="flex items-center gap-2">
      <Icon :d="mdiShapePlus" />
      <h2 class="text-xl font-semibold">分类</h2>
    </div>

    <ul>
      <li>
        <RouterLink
          class="flex items-center gap-2 w-auto px-2 py-1 rounded hover:bg-accent hover:text-accent-content"
          to="/"
        >
          <Icon :d="mdiDotsGrid" :size="20" />全部</RouterLink
        >
      </li>
      <li v-for="category in categories" :key="category.id">
        <RouterLink
          class="block w-auto px-2 py-1 rounded hover:bg-accent hover:text-accent-content"
          :to="`/?category=${category.name}`"
          >{{ category.name }}</RouterLink
        >
      </li>
    </ul>
  </div>
</template>
