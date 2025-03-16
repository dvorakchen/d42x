<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import qs from "qs";
import { getPaginatedMemeList, skeletonMemeList } from "../net/meme";
import MemeGroup from "./MemeGroup.vue";

const curPage = ref(0);
const totalPage = ref(1);
const loading = ref(false);
const memeList = ref(skeletonMemeList);

onMounted(async () => {
  await loadNextPage(() => {
    memeList.value = [];
  });
});

const canLoadMore = computed(() => {
  return totalPage.value > curPage.value;
});

async function loadNextPage(beforeLoad?: Function) {
  if (curPage.value >= totalPage.value) {
    return;
  }
  loading.value = true;
  curPage.value++;

  let fullQ = location.search;
  if (fullQ.startsWith("?")) {
    fullQ = fullQ.substring(1);
  }
  const q = qs.parse(fullQ);

  const paginatedList = await getPaginatedMemeList(
    curPage.value,
    (q.category ?? "") as string
  );
  curPage.value = paginatedList.page;
  totalPage.value = paginatedList.total;

  if (beforeLoad) {
    beforeLoad();
  }

  memeList.value.push(...paginatedList.list);

  loading.value = false;
}

async function loadMore() {
  await loadNextPage();
}
</script>

<template>
  <main class="flex flex-col gap-8 w-xl">
    <ul class="flex flex-col gap-6">
      <li v-for="meme in memeList" :key="meme.id">
        <MemeGroup :meme="meme" />
        <div class="divider"></div>
      </li>
    </ul>

    <div class="w-auto flex justify-center">
      <button
        class="btn btn-primary"
        @click="loadMore"
        :disabled="!canLoadMore || loading"
      >
        {{ canLoadMore ? "加载更多" : "没有更多了" }}
      </button>
    </div>
  </main>
</template>
