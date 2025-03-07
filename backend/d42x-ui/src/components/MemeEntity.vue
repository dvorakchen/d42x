<script setup lang="ts">
import type { MemeUrlEntityModel } from "../net/models";
import placeholderImg from "../assets/placeholder-img.png";
import { onMounted, ref } from "vue";

const props = defineProps<{
  entity: MemeUrlEntityModel;
}>();

const showUrl = ref(placeholderImg);
const loadFile = new Image();

onMounted(() => {
  loadFile.onload = () => {
    showUrl.value = props.entity.url;
  };
  loadFile.src = props.entity.url;
});

function handleZoom(ev: MouseEvent) {
  const img = ev.target as HTMLImageElement;
  img.classList.toggle("w-48");
  img.classList.toggle("max-h-64");
  img.classList.toggle("cursor-zoom-in");
  img.classList.toggle("cursor-zoom-out");
}
</script>

<template>
  <img
    class="w-48 max-h-64 object-contain rounded-lg cursor-zoom-in"
    :src="showUrl"
    alt="image"
    @click="handleZoom"
  />
</template>
