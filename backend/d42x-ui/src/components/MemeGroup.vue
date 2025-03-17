<script setup lang="ts">
import MemeEntity from "./MemeEntity.vue";
import { toYYYYMMDDHHmmss } from "../utilities/date";
import type { MemeEntityModel } from "../net/models";

defineProps<{
  meme: MemeEntityModel;
}>();
</script>

<template>
  <header class="flex flex-col gap-4 mb-2 text-sm">
    <div class="flex gap-2">
      <RouterLink
        :to="`/?category=${cate}`"
        class="badge badge-sm font-semibold text-nowrap odd:badge-primary even:badge-accent last:badge-info"
        v-for="cate in meme.categories"
        :key="cate"
        >{{ cate }}</RouterLink
      >
    </div>
    <div class="space-x-2">
      <span class="font-bold">{{ meme.nickname }}</span>
      <span>{{ toYYYYMMDDHHmmss(meme.show_date_time) }}</span>
    </div>
  </header>
  <div
    class="flex flex-col flex-wrap gap-3 p-3 w-fit min-w-52 min-h-32 border border-base-content rounded-lg"
  >
    <MemeEntity v-for="entity in meme.list" :key="entity.id" :entity="entity" />
  </div>
</template>
