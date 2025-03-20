<script setup lang="ts">
import { mdiEmail, mdiGithub } from "@mdi/js";
import Icon from "./Icon.vue";
</script>

<template>
  <footer
    class="footer sm:footer-horizontal bg-base-200 text-base-content p-10"
  >
    <aside class="space-y-4">
      <span class="text-5xl">🥰</span>
      <p class="space-y-1">
        Copyright © {{ new Date().getFullYear() }} - All right reserved
        <br />
        <a href="https://beian.miit.gov.cn/" target="_blank"
          >粤ICP备2024191084号-2</a
        >
      </p>
    </aside>
    <nav>
      <h6 class="footer-title">联系我</h6>
      <a
        class="link link-hover flex items-center gap-2"
        href="mailto:dvorakchen@outlook.com"
      >
        <Icon :d="mdiEmail" :size="20" />
        dvorakchen@outlook.com</a
      >
      <a class="link link-hover flex items-center gap-2">
        <Icon :d="mdiGithub" :size="20" />
        GitHub</a
      >
    </nav>
  </footer>
</template>
