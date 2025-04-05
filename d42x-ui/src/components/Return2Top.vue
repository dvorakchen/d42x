<script setup lang="ts">
function handleTop() {
  window.scrollTo({
    top: 0,
    behavior: "smooth",
  });
}
</script>

<template>
  <div class="fixed right-8 bottom-8 z-50">
    <button class="btn btn-primary btn-circle text-2xl" @click="handleTop">
      🚀
    </button>
  </div>
</template>
