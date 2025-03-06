<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getPaginatedMemeList } from "../net/meme";
import type { MemeEntityModel } from "../net/models";
import MemeSeries from "./MemeSeries.vue";

const curPage = ref(0);
const totalPage = ref(1);
const memeList = ref([] as MemeEntityModel[]);

onMounted(async () => {
  await loadNextPage();
});

const canLoadMore = computed(() => {
  return totalPage.value < curPage.value;
});

async function loadNextPage() {
  if (curPage.value >= totalPage.value) {
    return;
  }
  curPage.value++;

  const paginatedList = await getPaginatedMemeList(curPage.value, "");
  curPage.value = paginatedList.page;
  totalPage.value = paginatedList.total;

  memeList.value.push(...paginatedList.list);
}

async function loadMore() {
  await loadNextPage();
}
</script>

<template>
  <main class="p-2 flex flex-col gap-8 w-full">
    <ul class="flex flex-col gap-6">
      <li class="p-2" v-for="meme in memeList" :key="meme.id">
        <MemeSeries :meme="meme" />
      </li>
    </ul>

    <div class="w-auto flex justify-center">
      <button
        class="btn btn-primary"
        @click="loadMore"
        :disabled="!canLoadMore"
      >
        {{ canLoadMore ? "加载更多" : "没有更多了" }}
      </button>
    </div>
  </main>
</template>
