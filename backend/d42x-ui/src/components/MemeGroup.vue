<script setup lang="ts">
import MemeEntity from "./MemeEntity.vue";
import { toYYYYMMDDHHmmss } from "../utilities/date";
import type { MemeEntityModel } from "../net/models";
import Icon from "./Icon.vue";
import { mdiDotsHorizontal, mdiHeart, mdiHeartBroken } from "@mdi/js";

defineProps<{
  meme: MemeEntityModel;
}>();
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
  <div class="mt-4 space-x-4 flex">
    <span class="flex items-center w-16">
      <label class="swap swap-flip text-2xl">
        <input type="checkbox" />
        <div class="swap-on">❤️</div>
        <div class="swap-off flex items-center justify-center">
          <Icon :d="mdiHeart" />
        </div>
      </label>
      <span>{{ meme.likes }}</span>
    </span>
    <span class="flex items-center">
      <label class="swap swap-flip text-2xl">
        <input type="checkbox" />
        <div class="swap-on">💔</div>
        <div class="swap-off flex items-center justify-center">
          <Icon :d="mdiHeartBroken" :size="23" />
        </div>
      </label>
      <span>{{ meme.unlikes }}</span>
    </span>
  </div>
</template>
