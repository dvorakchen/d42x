<script setup lang="ts">
import CategoryList from "../components/CategoryList.vue";
import MemeList from "../components/MemeList.vue";
</script>

<template>
  <div class="w-full max-w-5xl m-auto pt-16 pb-16 px-4">
    <h1 class="text-4xl font-bold">
      D42X
      <span class="text-sm font-medium">- 呼唤爱与和平</span>
    </h1>
    <div class="pt-16 flex gap-4">
      <section class="hidden md:block min-w-48 w-48">
        <CategoryList />
      </section>

      <section class="flex grow">
        <MemeList />
      </section>

      <section class="hidden lg:block min-w-48 w-48"></section>
    </div>
  </div>
  <footer
    class="footer sm:footer-horizontal footer-center bg-base-300 text-base-content p-4"
  >
    <aside>
      <p>
        Copyright © {{ new Date().getFullYear() }} - All right reserved by D42X
        Dvorak
      </p>
      <p>
        <a href="https://beian.miit.gov.cn/" target="_blank"
          >粤ICP备2024191084号-2</a
        >
      </p>
    </aside>
  </footer>
</template>
