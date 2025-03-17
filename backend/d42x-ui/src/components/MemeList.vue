<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { getPaginatedMemeList, skeletonMemeList } from "../net/meme";
import MemeGroup from "./MemeGroup.vue";
import { useRoute } from "vue-router";

const curPage = ref(0);
const totalPage = ref(1);
const loading = ref(false);
const memeList = ref(skeletonMemeList);

const route = useRoute();

onMounted(async () => {
  await loadNextPage(() => {
    memeList.value = [];
  });
});

watch(
  () => route.query,
  async () => {
    curPage.value = 0;
    await loadNextPage(() => {
      memeList.value = [];
    });
  },
  { immediate: true }
);

const canLoadMore = computed(() => {
  return totalPage.value > curPage.value;
});

async function loadNextPage(beforeLoad?: Function) {
  if (curPage.value >= totalPage.value) {
    return;
  }
  loading.value = true;
  curPage.value++;

  const paginatedList = await getPaginatedMemeList(
    curPage.value,
    (route.query.category ?? "") as string
  );
  curPage.value = paginatedList.page;
  totalPage.value = paginatedList.total > 0 ? paginatedList.total : 1;

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
