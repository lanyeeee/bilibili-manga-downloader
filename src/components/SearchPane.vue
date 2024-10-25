<script setup lang="ts">
import {ref, computed} from "vue";
import {commands, Manga, SearchMangaRespData} from "../bindings.ts";
import {useNotification} from "naive-ui";
import MangaCard from "./MangaCard.vue";

const notification = useNotification();

const searchMangaRespData = ref<SearchMangaRespData>();
const currentTabName = defineModel<"search" | "episode">("currentTabName", {required: true});
const selectedManga = defineModel<Manga | undefined>("selectedManga", {required: true});

const searchInput = ref<string>("");
const mangaIdInput = ref<string>("");
const searchPage = ref<number>(1);

const searchPageCount = computed(() => {
  if (searchMangaRespData.value === undefined) {
    return 0;
  }
  const total = searchMangaRespData.value.total_num;
  return Math.floor(total / 20) + 1;
});

async function searchByKeyword(keyword: string, pageNum: number) {
  searchPage.value = pageNum;
  let result = await commands.searchManga(keyword, pageNum);
  if (result.status === "error") {
    notification.error({title: "搜索失败", description: result.error});
    return;
  }

  searchMangaRespData.value = result.data;
  console.log("searchData", searchMangaRespData.value);
}

async function searchById(id: number) {
  let result = await commands.getManga(id);
  if (result.status === "error") {
    notification.error({title: "获取漫画章节详情失败", description: result.error});
    return;
  }

  currentTabName.value = "episode";
  searchMangaRespData.value = undefined;
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
          漫画名:
        </template>
      </n-input>
      <n-button size="tiny" @click="searchByKeyword(searchInput.trim(), 1)">搜索</n-button>
      <div class="min-w-2"></div>
      <n-input class="text-align-left"
               size="tiny"
               v-model:value="mangaIdInput"
               placeholder=""
               clearable
               @keydown.enter="searchById(Number(mangaIdInput.trim()))"
      >
        <template #prefix>
          漫画ID:
        </template>
      </n-input>
      <n-button size="tiny" @click="searchById(Number(mangaIdInput.trim()))">直达</n-button>
    </div>
    <div v-if="searchMangaRespData!==undefined" class="flex flex-col gap-row-1 overflow-auto p-2">
      <div class="flex flex-col gap-row-2 overflow-auto">
        <manga-card v-for="mangaInSearch in searchMangaRespData.list"
                    :key="mangaInSearch.id"
                    :manga-info="mangaInSearch"
                    v-model:current-tab-name="currentTabName"
                    v-model:selected-manga="selectedManga"/>
      </div>
      <n-pagination :page-count="searchPageCount"
                    :page="searchPage"
                    @update:page="searchByKeyword(searchInput.trim(), $event)"/>
    </div>
  </div>
</template>