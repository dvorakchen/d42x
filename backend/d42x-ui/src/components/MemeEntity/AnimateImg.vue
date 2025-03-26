<script setup lang="ts">
import { computed, onMounted, ref, useTemplateRef } from "vue";
import type { MemeUrlEntityModel } from "../../net/models";
import Icon from "../Icon.vue";
import { mdiPlayCircle } from "@mdi/js";

const props = defineProps<{
  entity: MemeUrlEntityModel;
}>();

const img = useTemplateRef("show-img");

const playing = ref(false);
const loading = ref(false);

onMounted(() => {
  if (img.value === null) {
    return;
  }

  img.value.onload = () => {
    loading.value = false;
  };
});

const showUrl = computed(() => {
  loading.value = true;
  return playing.value ? props.entity.url : props.entity.cover;
});

function handleTrigglePlay() {
  playing.value = !playing.value;
}
</script>

<template>
  <div class="w-full flex justify-center">
    <div class="relative cursor-pointer" @click="handleTrigglePlay">
      <img
        ref="show-img"
        class="object-contain rounded-lg"
        :class="{
          'max-h-96': !playing,
        }"
        :src="showUrl"
        alt="image"
      />
      <div
        class="absolute left-2 bottom-2 flex items-center justify-center"
        v-show="!playing || loading"
      >
        <Icon :d="mdiPlayCircle" :size="50" color="#222" v-if="!playing" />
        <span class="loading loading-dots bg-base-200" v-if="loading"></span>
      </div>
    </div>
  </div>
</template>
