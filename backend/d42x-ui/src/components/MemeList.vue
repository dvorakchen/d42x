<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { getPaginatedMemeList, skeletonMemeList } from "../net/meme";
import MemeGroup from "./MemeGroup.vue";
import { useRoute } from "vue-router";
import { getInteractions } from "../net/interactions";

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

  const ids = paginatedList.list.map((item) => item.id);
  const interactions = await getInteractions(ids);

  memeList.value.push(...paginatedList.list);

  for (const item of interactions) {
    const meme = memeList.value.find((t) => t.id === item.id);
    if (meme === undefined) {
      continue;
    }
    meme.likes = item.likes;
    meme.unlikes = item.unlikes;
  }

  loading.value = false;
}

async function loadMore() {
  await loadNextPage();
}
</script>

<template>
  <main class="flex flex-col gap-8 lg:w-xl max-w-full">
    <ul class="flex flex-col gap-1">
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
