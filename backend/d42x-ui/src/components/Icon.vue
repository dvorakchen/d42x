<script setup lang="ts">
defineProps<{
  d: string;
  size?: number;
  color?: string;
}>();

const DEFAULT_SIZE = 24;
</script>

<template>
  <svg
    viewBox="0 0 24 24"
    :width="size ?? DEFAULT_SIZE"
    :height="size ?? DEFAULT_SIZE"
    class="inline-block"
    :color="color"
  >
    <path fill="currentColor" :d="d" />
  </svg>
</template>
