<script setup lang="ts">
import MemeEntity from "./MemeEntity/Index.vue";
import { toYYYYMMDDHHmmss } from "../utilities/date";
import type { MemeEntityModel } from "../net/models";
import Icon from "./Icon.vue";
import {
  mdiCheck,
  mdiDotsHorizontal,
  mdiHeart,
  mdiHeartBroken,
  mdiLinkVariant,
  mdiShareAll,
} from "@mdi/js";
import { onMounted, ref } from "vue";
import { useInteractStore, type Interaction } from "../stores/interaction";
import { serverApi } from "../net/http";

// TODO
// const emit = defineEmits<{
//   (e: "editCategory", memeId: string): void;
// }>();

const props = defineProps<{
  meme: MemeEntityModel;
}>();

const interactStore = useInteractStore();
const expandShare = ref(false);
const linkCopied = ref(false);
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

function handleShare() {
  expandShare.value = !expandShare.value;
}

async function handleCopyLink() {
  await navigator.clipboard.writeText(
    `${location.origin}/memes/${props.meme.short_id}`
  );

  linkCopied.value = true;
}

// TODO
// function handleEtitCategory() {
//   emit("editCategory", props.meme.id);
// }
</script>

<template>
  <header class="flex flex-col gap-2 mb-2 text-sm">
    <div class="flex gap-2">
      <div class="flex-grow flex items-center gap-1 flex-wrap">
        <RouterLink
          :to="`/?category=${cate}`"
          class="badge badge-sm font-semibold text-nowrap odd:badge-primary even:badge-accent last:badge-info"
          v-for="cate in meme.categories"
          :key="cate"
          >{{ cate }}</RouterLink
        >
      </div>

      <button class="btn">
        <RouterLink :to="`/memes/${props.meme.short_id}`"> 详情页 </RouterLink>
      </button>
      
      <div class="dropdown dropdown-end">
        <button class="btn btn-ghost btn-square" tabindex="0" role="button">
          <Icon :d="mdiDotsHorizontal" />
        </button>
        <ul
          tabindex="0"
          class="dropdown-content menu bg-info text-info-content font-bold rounded-box z-1 w-52 p-2 shadow-sm"
        >
          <!-- TODO -->
          <!-- <li><a @click="handleEtitCategory">🚩标签建议</a></li> -->
        </ul>
      </div>
    </div>
    <div class="space-x-2">
      <span class="font-bold">{{ meme.nickname }}</span>
      <span>{{ toYYYYMMDDHHmmss(meme.show_date_time) }}</span>
    </div>
  </header>
  <div
    class="flex flex-col justify-center gap-2 p-2 w-full min-h-64 bg-black border border-base-content rounded-lg"
  >
    <MemeEntity v-for="entity in meme.list" :key="entity.id" :entity="entity" />
  </div>
  <!-- interactions -->
  <div class="mt-4 space-x-4 flex flex-wrap">
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
    <div class="mx-4 flex items-center">
      <button
        class="btn btn-ghost btn-xs tooltip"
        data-tip="分享"
        @click="handleShare"
      >
        <Icon :d="mdiShareAll" />
      </button>
      <div
        class="grid transition-all"
        :class="{
          'grid-cols-[1fr]': expandShare,
          'grid-cols-[0fr]': !expandShare,
        }"
      >
        <div class="join overflow-hidden transition-all">
          <button
            class="btn btn-xs btn-primary join-item"
            @click="handleCopyLink"
          >
            <Icon :d="mdiLinkVariant" :size="20" />
            复制链接
            <Icon :d="mdiCheck" v-if="linkCopied" />
          </button>
        </div>
      </div>
    </div>
    <button class="btn btn-sm">
      <RouterLink :to="`/memes/${props.meme.short_id}`"> 详情页 </RouterLink>
    </button>
  </div>
</template>
