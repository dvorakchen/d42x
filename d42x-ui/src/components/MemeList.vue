<script setup lang="ts">
import { computed, onMounted, reactive, ref, useTemplateRef, watch } from "vue";
import { getPaginatedMemeList, skeletonMemeList } from "../net/meme";
import MemeGroup from "./MemeGroup.vue";
import { useRoute } from "vue-router";
import { getInteractions } from "../net/interactions";
import EditCategory from "./EditCategory.vue";
import type { MemeEntityModel } from "../net/models";
import { useMsgStore } from "../stores/msgStore";
import { heroColors } from "../utilities/hero-color";

const dialogEditCategory = useTemplateRef("dialog_edit_category");
const editCategoryMeme = ref(null as MemeEntityModel | null);

const curPage = ref(0);
const totalPage = ref(1);
const loading = ref(false);
const memeList = ref(skeletonMemeList);

const route = useRoute();
const msgStore = useMsgStore();

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
    randomHeroColor = heroColors[Math.floor(Math.random() * heroColors.length)];
  },
  { immediate: true }
);

let randomHeroColor = reactive(
  heroColors[Math.floor(Math.random() * heroColors.length)]
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

// function handleEtitCategory(memeId: string) {
//   editCategoryMeme.value = memeList.value.find((t) => t.id === memeId) ?? null;
//   dialogEditCategory.value!.checked = true;
// }

function handleCancelEditCategory() {
  editCategoryMeme.value = null;
  dialogEditCategory.value!.checked = false;
}

function handleAfterEditCategory(_list: string[]) {
  editCategoryMeme.value = null;
  dialogEditCategory.value!.checked = false;

  msgStore.push({
    color: "success",
    value: "提交成功，感谢您的建议",
  });
}
</script>

<template>
  <main class="flex flex-col gap-8 lg:w-2xl max-w-full mb-16">
    <div class="hero h-32 rounded-xl" :style="randomHeroColor">
      <div class="hero-content text-3xl font-bold">
        {{ route.query.category ?? "大家开心最重要" }}
      </div>
    </div>

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

  <input
    type="checkbox"
    ref="dialog_edit_category"
    id="dialog_edit_category"
    class="modal-toggle"
  />
  <div class="modal" role="dialog" v-if="editCategoryMeme">
    <div
      class="modal-box md:-mt-16 max-w-full w-full h-full md:h-auto md:max-h-[80vh] md:w-3xl space-y-4"
    >
      <h3 class="text-lg font-bold">写下你的标签建议</h3>
      <EditCategory
        :meme="editCategoryMeme"
        @cancel="handleCancelEditCategory"
        @afterSubmit="handleAfterEditCategory"
      />
    </div>
    <label class="modal-backdrop" for="dialog_edit_category">Close</label>
  </div>
</template>
