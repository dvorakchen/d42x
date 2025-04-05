<script setup lang="ts">
import { useMsgStore } from "../stores/msgStore";

const msgStore = useMsgStore();
</script>

<template>
  <div class="toast toast-top toast-end z-50">
    <div
      class="alert"
      :class="{
        'alert-success': msg.color === 'success',
        'alert-info': msg.color === 'info',
        'alert-warning': msg.color === 'warning',
        'alert-error': msg.color === 'error',
      }"
      v-for="msg in msgStore.msgs"
    >
      <span class="font-bold">{{ msg.value }}</span>
    </div>
  </div>
</template>
