<script setup lang="ts">
import MemeEntity from "./MemeEntity.vue";
import { toYYYYMMDDHHmmss } from "../utilities/date";
import type { MemeEntityModel } from "../net/models";
import Icon from "./Icon.vue";
import { mdiDotsHorizontal, mdiHeart, mdiHeartBroken } from "@mdi/js";
import { onMounted, ref } from "vue";
import { useInteractStore, type Interaction } from "../stores/interaction";
import { serverApi } from "../net/http";

const props = defineProps<{
  meme: MemeEntityModel;
}>();

const interactStore = useInteractStore();
const like = ref(false);
const unlike = ref(false);
const liked = ref(false);
const unliked = ref(false);
const interactRecord = ref(null as Interaction | null);

onMounted(() => {
  interactRecord.value = interactStore.getRecord(props.meme.id) ?? {
    like: false,
    unlike: false,
  };

  like.value = interactRecord.value.like;
  unlike.value = interactRecord.value.unlike;
});

async function handleLike() {
  if (like.value) {
    liked.value = true;
  }
  if (
    interactRecord.value === null ||
    interactRecord.value.like ||
    like.value
  ) {
    return;
  }
  like.value = true;
  props.meme.likes++;

  const resp = await serverApi.put(`memes/${props.meme.id}/like`);
  if (resp.status === 200) {
    interactStore.like(props.meme.id, like.value);
  }
}

async function handleUnlike() {
  if (unlike.value) {
    unliked.value = true;
  }
  if (
    interactRecord.value === null ||
    interactRecord.value.unlike ||
    unlike.value
  ) {
    return;
  }
  unlike.value = true;
  props.meme.unlikes++;

  const resp = await serverApi.put(`memes/${props.meme.id}/unlike`);
  if (resp.status === 200) {
    interactStore.unlike(props.meme.id, unlike.value);
  }
}
</script>

<template>
  <header class="flex flex-col gap-4 mb-2 text-sm">
    <div class="flex gap-2">
      <div class="flex-grow space-x-1">
        <RouterLink
          :to="`/?category=${cate}`"
          class="badge badge-sm font-semibold text-nowrap odd:badge-primary even:badge-accent last:badge-info"
          v-for="cate in meme.categories"
          :key="cate"
          >{{ cate }}</RouterLink
        >
      </div>
      <button class="btn btn-ghost btn-square">
        <Icon :d="mdiDotsHorizontal" />
      </button>
    </div>
    <div class="space-x-2">
      <span class="font-bold">{{ meme.nickname }}</span>
      <span>{{ toYYYYMMDDHHmmss(meme.show_date_time) }}</span>
    </div>
  </header>
  <div
    class="flex flex-col gap-2 p-2 w-full min-h-64 bg-black border border-base-content rounded-lg"
  >
    <MemeEntity v-for="entity in meme.list" :key="entity.id" :entity="entity" />
  </div>
  <!-- interactions -->
  <div class="mt-4 space-x-4 flex">
    <span class="flex items-center gap-2 w-16"
      ><div class="cursor-pointer relative" @click="handleLike">
        <span
          v-if="like"
          class="absolute inset-0 animate-[ping_1s_cubic-bezier(0,0,0.2,1)_none]"
          :class="{ tooltip: liked }"
          data-tip="你已经❤️过了"
        >
          <Icon :d="mdiHeart" :color="'red'" />
        </span>
        <Icon :d="mdiHeart" :color="like ? 'red' : ''" />
      </div>
      <span>{{ meme.likes }}</span>
    </span>
    <span class="flex items-center">
      <div class="cursor-pointer relative" @click="handleUnlike">
        <span
          v-if="unlike"
          class="absolute inset-0 animate-[ping_1s_cubic-bezier(0,0,0.2,1)_none]"
          :class="{ tooltip: unliked }"
          data-tip="你已经💔过了"
        >
          <Icon :d="mdiHeartBroken" :color="'grey'" />
        </span>
        <Icon :d="mdiHeartBroken" :color="unlike ? 'grey' : ''" />
      </div>
      <span>{{ meme.unlikes }}</span>
    </span>
  </div>
</template>
