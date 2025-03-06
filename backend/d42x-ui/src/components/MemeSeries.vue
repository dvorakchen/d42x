<script setup lang="ts">
import MemeEntity from "./MemeEntity.vue";
import { toYYYYMMDDHHmmss } from "../utilities/date";
import type { MemeEntityModel } from "../net/models";

defineProps<{
  meme: MemeEntityModel;
}>();
</script>

<template>
  <div class="flex gap-4 mb-2 text-sm">
    <span class="font-bold">{{ meme.nickname }}</span>
    <span>{{ toYYYYMMDDHHmmss(meme.show_date_time) }}</span>
  </div>
  <div
    class="flex flex-col flex-wrap gap-3 p-3 w-fit min-w-52 min-h-32 border border-base-content rounded-lg"
  >
    <MemeEntity v-for="entity in meme.list" :key="entity.id" :entity="entity" />
  </div>
</template>
