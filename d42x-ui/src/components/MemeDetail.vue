<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import type { MemeEntityModel } from "../net/models";
import { getMemeDetail } from "../net/meme";
import MemeGroup from "./MemeGroup.vue";

const route = useRoute();

const loading = ref(false);
const model = ref(null as MemeEntityModel | null);

onMounted(async () => {
  loading.value = true;
  const shortId: string = route.params.id as string;
  if (shortId === "") {
    return;
  }
  model.value = await getMemeDetail(shortId);
  loading.value = false;
});
</script>

<template>
  <div v-if="loading">
    <div class="flex w-full flex-col gap-4">
      <div class="skeleton h-4 w-full"></div>
      <div class="skeleton h-4 w-28"></div>
      <div class="skeleton h-64 w-full"></div>
      <div class="flex gap-4">
        <span class="skeleton h-4 w-12"></span>
        <span class="skeleton h-4 w-12"></span>
        <span class="skeleton h-4 w-12"></span>
      </div>
    </div>
  </div>
  <div v-else>
    <MemeGroup :meme="model" v-if="model" />
  </div>
</template>
