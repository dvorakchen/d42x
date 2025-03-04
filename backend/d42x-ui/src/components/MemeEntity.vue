<script setup lang="ts">
import { onMounted, ref } from "vue";
import type { MemeEntityModel } from "../net/models";
import { toYYYYMMDDHHmmss } from "../utilities/date";
import placeholderImg from "../assets/placeholder-img.png";

const props = defineProps<{
  meme: MemeEntityModel;
}>();

const loading = ref(true);
const imgUrl = ref(placeholderImg);

const displayImg = new Image();

onMounted(() => {
  displayImg.onload = () => {
    imgUrl.value = displayImg.src;
    loading.value = false;
  };

  displayImg.src = props.meme.url;
});

function handleZoom(ev: MouseEvent) {
  const img = ev.target as HTMLImageElement;
  img.classList.toggle("w-48");
  img.classList.toggle("cursor-zoom-in");
  img.classList.toggle("cursor-zoom-out");
}
</script>

<template>
  <div class="flex gap-4 mb-2 text-sm">
    <span class="font-bold">{{ meme.nickname }}</span>
    <span>{{ toYYYYMMDDHHmmss(meme.show_date_time) }}</span>
  </div>
  <div class="p-3 w-fit border border-base-content rounded-lg">
    <img
      class="w-48 object-contain rounded-lg cursor-zoom-in"
      :src="imgUrl"
      alt="image"
      @click="handleZoom"
    />
  </div>
</template>
