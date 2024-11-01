<script setup lang="ts">
import {computed, ref} from "vue";
import {Comic, commands, SearchRespData} from "../bindings.ts";
import {useNotification} from "naive-ui";
import ComicCard from "./ComicCard.vue";

const notification = useNotification();

const searchRespData = ref<SearchRespData>();
const currentTabName = defineModel<"search" | "episode">("currentTabName", {required: true});
const selectedComic = defineModel<Comic | undefined>("selectedComic", {required: true});

const searchInput = ref<string>("");
const comicIdInput = ref<string>("");
const searchPage = ref<number>(1);

const searchPageCount = computed(() => {
  if (searchRespData.value === undefined) {
    return 0;
  }
  const total = searchRespData.value.comic_data.total_num;
  return Math.floor(total / 20) + 1;
});

async function searchByKeyword(keyword: string, pageNum: number) {
  searchPage.value = pageNum;
  let result = await commands.search(keyword, pageNum);
  if (result.status === "error") {
    notification.error({title: "搜索失败", description: result.error});
    return;
  }

  searchRespData.value = result.data;
  console.log("searchData", searchRespData.value);
}

async function searchById(comicId: number) {
  let result = await commands.getComic(comicId);
  if (result.status === "error") {
    notification.error({title: "获取漫画章节详情失败", description: result.error});
    return;
  }

  selectedComic.value = result.data;
  currentTabName.value = "episode";
  searchRespData.value = undefined;
}

</script>

<template>
  <div class="h-full flex flex-col">
    <div class="flex">
      <n-input class="text-align-left"
               size="tiny"
               v-model:value="searchInput"
               placeholder=""
               clearable
               @keydown.enter="searchByKeyword(searchInput.trim(), 1)"
      >
        <template #prefix>
          关键词:
        </template>
      </n-input>
      <n-button size="tiny" @click="searchByKeyword(searchInput.trim(), 1)">搜索</n-button>
      <div class="min-w-2"></div>
      <n-input class="text-align-left"
               size="tiny"
               v-model:value="comicIdInput"
               placeholder=""
               clearable
               @keydown.enter="searchById(Number(comicIdInput.trim()))"
      >
        <template #prefix>
          漫画ID:
        </template>
      </n-input>
      <n-button size="tiny" @click="searchById(Number(comicIdInput.trim()))">直达</n-button>
    </div>
    <div v-if="searchRespData!==undefined" class="flex flex-col gap-row-1 overflow-auto p-2">
      <div class="flex flex-col gap-row-2 overflow-auto">
        <comic-card v-for="comicInSearch in searchRespData.comic_data.list"
                    :key="comicInSearch.id"
                    :comic-info="comicInSearch"
                    v-model:current-tab-name="currentTabName"
                    v-model:selected-comic="selectedComic"/>
      </div>
      <n-pagination :page-count="searchPageCount"
                    :page="searchPage"
                    @update:page="searchByKeyword(searchInput.trim(), $event)"/>
    </div>
  </div>
</template>