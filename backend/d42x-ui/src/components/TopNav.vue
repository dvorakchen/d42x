<script setup lang="ts">
import { onMounted, ref } from "vue";
import Icon from "../components/Icon.vue";
import CategoryList from "./CategoryList.vue";
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
  <nav>
    <h1 class="text-4xl font-bold">
      <RouterLink to="/">🤣D42X</RouterLink>
      <span class="text-sm font-medium"> - {{ showSlug }}</span>
    </h1>
    <div class="md:hidden pt-8 pb-4">
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
    </div>
  </nav>
</template>
