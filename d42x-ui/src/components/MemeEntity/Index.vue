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
    v-else-if="
      props.entity.format === AllowMemeFormats.GIF ||
      (props.entity.format === AllowMemeFormats.WEBP &&
        props.entity.cover !== '')
    "
  />
  <StaticImg :entity="entity" v-else />
</template>
