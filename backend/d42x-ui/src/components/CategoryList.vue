<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getCategoryList } from "../net/category";
import type { CategoryModel } from "../net/models";

const categories = ref([] as CategoryModel[]);

onMounted(async () => {
  categories.value = await getCategoryList();
});
</script>

<template>
  <div class="flex flex-col gap-4 h-full py-4 border-r border-base-content">
    <h2 class="text-xl font-semibold">分类</h2>
    <ul>
      <li v-for="category in categories" :key="category.id">
        <a :href="`/?category=${category.name}`">{{ category.name }}</a>
      </li>
    </ul>
  </div>
</template>
