<script setup lang="ts">
import { mdiClose, mdiTagEdit } from "@mdi/js";
import type { MemeEntityModel } from "../net/models";
import Icon from "./Icon.vue";
import { ref, useTemplateRef } from "vue";
import { categorySuggest } from "../net/category";

const emit = defineEmits<{
  (e: "cancel"): void;
  (e: "afterSubmit", list: string[]): void;
}>();

const props = defineProps<{
  meme: MemeEntityModel;
}>();

const inputRef = useTemplateRef("input-new-category");
const list = ref(new Set(props.meme.categories));

function handleRemove(name: string) {
  list.value.delete(name);
}

function handleEnter(ev: KeyboardEvent) {
  const ele = ev.target as HTMLInputElement;
  const text = ele.value?.trim() ?? "";
  addNewCategories(text);

  ele.value = "";
}

function handleEnterClick() {
  const text = inputRef.value!.value?.trim() ?? "";
  addNewCategories(text);

  inputRef.value!.value = "";
}

function addNewCategories(text: string) {
  if (text === "") {
    return;
  }

  const new_list = text.split(" ").filter((t) => t !== "");
  for (const item of new_list) {
    list.value.add(item);
  }
}

async function handleSubmitChange() {
  await categorySuggest(props.meme.id, Array.from(list.value));

  emit("afterSubmit", Array.from(list.value));
}

function handleCancelChange() {
  emit("cancel");
}
</script>

<template>
  <div class="flex flex-col-reverse md:flex-row gap-4 w-full h-[10/12]">
    <section class="max-h-[500px] overflow-scroll">
      <div
        class="flex flex-col relative w-auto max-w-44 max-h-full rounded-xl overflow-scroll"
      >
        <img v-for="img in meme.list" :src="img.url" alt="Img" />
      </div>
    </section>
    <section class="space-y-4 max-h-full">
      <div class="flex flex-wrap gap-2">
        <span
          v-for="cate in list"
          class="badge badge-accent odd:badge-info font-bold"
        >
          {{ cate }}
          <button
            class="btn btn-xs btn-circle w-5 h-5"
            @click="handleRemove(cate)"
          >
            <Icon :d="mdiClose" :size="10" />
          </button>
        </span>
      </div>
      <div class="space-y-2">
        <p class="text-sm">可以输入建议的标签，多个标签用空格分隔</p>
        <label class="input">
          <Icon :d="mdiTagEdit" />
          <input
            ref="input-new-category"
            type="search"
            class="grow"
            placeholder="建议的标签"
            @keydown.enter="handleEnter"
          />
          <button class="btn btn-xs btn-ghost" @click="handleEnterClick">
            <kbd class="kbd kbd-sm">回车</kbd>
          </button>
        </label>
      </div>
      <div class="flex justify-end gap-4">
        <button class="btn btn-secondary" @click="handleCancelChange">
          取消
        </button>
        <button class="btn btn-primary" @click="handleSubmitChange">
          提交建议
        </button>
      </div>
    </section>
  </div>
</template>
