<script setup lang="ts">
import { AllowMemeFormats } from "../../config";
import type { MemeUrlEntityModel } from "../../net/models";
import AnimateImg from "./AnimateImg.vue";
import StaticImg from "./StaticImg.vue";

const props = defineProps<{
  entity: MemeUrlEntityModel;
}>();

const STATIC_FORMATS = new Set([
  AllowMemeFormats.JPEG,
  AllowMemeFormats.JPG,
  AllowMemeFormats.PNG,
]);
</script>

<template>
  <StaticImg :entity="entity" v-if="STATIC_FORMATS.has(props.entity.format)" />
  <AnimateImg
    :entity="entity"
    v-if="props.entity.format === AllowMemeFormats.GIF"
  />
</template>
