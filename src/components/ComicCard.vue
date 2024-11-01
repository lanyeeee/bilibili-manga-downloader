<script setup lang="ts">
import {ComicInfo} from "../types.ts";
import {commands, Comic} from "../bindings.ts";
import {useNotification} from "naive-ui";

const notification = useNotification();

defineProps<{
  comicInfo: ComicInfo;
}>();

const currentTabName = defineModel<"search" | "episode">("currentTabName", {required: true});
const selectedComic = defineModel<Comic | undefined>("selectedComic", {required: true});

async function onClickItem(comicId: number) {
  const result = await commands.getComic(comicId);
  if (result.status === "error") {
    notification.error({title: "获取漫画详情失败", description: result.error});
    return;
  }
  selectedComic.value = result.data;
  currentTabName.value = "episode";
}

</script>

<template>
  <n-card content-style="padding: 0.25rem;" hoverable>
    <div class="flex">
      <img
          class="w-24 aspect-[3/4] object-contain mr-4 cursor-pointer transform transition-transform duration-200 hover:scale-106"
          :src="comicInfo.vertical_cover"
          alt=""
          referrerpolicy="no-referrer"
          @click="onClickItem(comicInfo.id)"/>
      <div class="flex flex-col">
        <span v-html="comicInfo.title"
              class="font-bold text-xl line-clamp-2 cursor-pointer transition-colors duration-200 hover:text-blue-5"
              @click="onClickItem(comicInfo.id)"/>
        <span v-html="comicInfo.author_name"/>
        <span v-html="comicInfo.styles"/>
        <span>{{ comicInfo.is_finish ? "已完结" : "连载中" }}</span>
      </div>
    </div>
  </n-card>
</template>

<style scoped>
:deep(.keyword) {
  @apply not-italic text-blue-400
}
</style>