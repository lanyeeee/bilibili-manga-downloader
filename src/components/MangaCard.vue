<script setup lang="ts">
import {MangaInfo} from "../types.ts";
import {commands, Manga} from "../bindings.ts";
import {useNotification} from "naive-ui";

const notification = useNotification();

defineProps<{
  mangaInfo: MangaInfo;
}>();

const currentTabName = defineModel<"search" | "episode">("currentTabName", {required: true});
const selectedManga = defineModel<Manga | undefined>("selectedManga", {required: true});

async function onClickItem(mangaId: number) {
  const result = await commands.getManga(mangaId);
  if (result.status === "error") {
    notification.error({title: "获取漫画详情失败", description: result.error});
    return;
  }
  selectedManga.value = result.data;
  currentTabName.value = "episode";
}

</script>

<template>
  <n-card content-style="padding: 0.25rem;" hoverable>
    <div class="flex">
      <img
          class="w-24 aspect-[3/4] object-contain mr-4 cursor-pointer transform transition-transform duration-200 hover:scale-106"
          :src="mangaInfo.vertical_cover"
          alt=""
          referrerpolicy="no-referrer"
          @click="onClickItem(mangaInfo.id)"/>
      <div class="flex flex-col">
        <span v-html="mangaInfo.title"
              class="font-bold text-xl line-clamp-2 cursor-pointer transition-colors duration-200 hover:text-blue-5"
              @click="onClickItem(mangaInfo.id)"/>
        <span v-html="mangaInfo.author_name"/>
        <span v-html="mangaInfo.styles"/>
        <span>{{ mangaInfo.is_finish ? "已完结" : "连载中" }}</span>
      </div>
    </div>
  </n-card>
</template>

<style scoped>
:deep(.keyword) {
  @apply not-italic text-blue-400
}
</style>