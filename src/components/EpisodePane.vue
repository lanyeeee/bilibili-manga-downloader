<script setup lang="ts">
import {SelectionArea, SelectionEvent, SelectionOptions} from "@viselect/vue";
import {nextTick, ref, watch} from "vue";
import {commands, Manga} from "../bindings.ts";
import {useNotification} from "naive-ui";

const notification = useNotification();

const selectedManga = defineModel<Manga | undefined>("selectedManga", {required: true});

const dropdownX = ref<number>(0);
const dropdownY = ref<number>(0);
const showDropdown = ref<boolean>(false);
const dropdownOptions = [
  {label: "勾选", key: "check"},
  {label: "取消勾选", key: "uncheck"},
  {label: "全选", key: "check all"},
  {label: "取消全选", key: "uncheck all"},
];
const checkedIds = ref<number[]>([]);
const selectedIds = ref<Set<number>>(new Set());
const selectionAreaRef = ref<InstanceType<typeof SelectionArea>>();

watch(selectedManga, () => {
  checkedIds.value = [];
  selectedIds.value.clear();
  selectionAreaRef.value?.selection?.clearSelection();
});

function extractIds(elements: Element[]): number[] {
  return elements.map(element => element.getAttribute("data-key"))
      .filter(Boolean)
      .map(Number)
      .filter(id => {
        const ep = selectedManga.value?.episodeInfos.find(ep => ep.episodeId === id);
        if (ep === undefined) {
          return false;
        }
        return !ep.isLocked && !ep.isDownloaded;
      });
}

function onDragStart({event, selection}: SelectionEvent) {
  if (!event?.ctrlKey && !event?.metaKey) {
    selection.clearSelection();
    selectedIds.value.clear();
  }
}

function onDragMove({store: {changed: {added, removed}}}: SelectionEvent) {
  extractIds(added).forEach(id => selectedIds.value.add(id));
  extractIds(removed).forEach(id => selectedIds.value.delete(id));
}

function onDropdownSelect(key: "check" | "uncheck" | "check all" | "uncheck all") {
  showDropdown.value = false;
  if (key === "check") {
    // 只有未勾选的才会被勾选
    [...selectedIds.value]
        .filter(id => !checkedIds.value.includes(id))
        .forEach(id => checkedIds.value.push(id));
  } else if (key === "uncheck") {
    checkedIds.value = checkedIds.value.filter(id => !selectedIds.value.has(id));
  } else if (key === "check all") {
    // 只有未锁定的才会被勾选
    selectedManga.value?.episodeInfos
        .filter(ep => !ep.isLocked && !ep.isDownloaded && !checkedIds.value.includes(ep.episodeId))
        .forEach(ep => checkedIds.value.push(ep.episodeId));
  } else if (key === "uncheck all") {
    checkedIds.value.length = 0;
  }
}

async function onContextMenu(e: MouseEvent) {
  showDropdown.value = false;
  await nextTick();
  showDropdown.value = true;
  dropdownX.value = e.clientX;
  dropdownY.value = e.clientY;
}

async function downloadEpisodes() {
  const episodesToDownload = selectedManga.value?.episodeInfos.filter(ep => !ep.isDownloaded && checkedIds.value.includes(ep.episodeId));
  if (episodesToDownload === undefined) {
    return;
  }
  const result = await commands.downloadEpisodes(episodesToDownload);
  if (result.status === "error") {
    console.error(result.error);
    return;
  }

  for (const downloadedEp of episodesToDownload) {
    const episode = selectedManga.value?.episodeInfos.find(ep => ep.episodeId === downloadedEp.episodeId);
    if (episode !== undefined) {
      episode.isDownloaded = true;
      checkedIds.value = checkedIds.value.filter(id => id !== downloadedEp.episodeId);
    }
  }
}

async function refreshEpisodes() {
  if (selectedManga.value === undefined) {
    return;
  }
  const result = await commands.getManga(selectedManga.value.id);
  if (result.status === "error") {
    notification.error({title: "获取漫画章节详情失败", description: result.error});
    return;
  }
  selectedManga.value = result.data;
}

</script>

<template>
  <div class="h-full flex flex-col">
    <div class="flex flex-justify-around">
      <span>总章数：{{ selectedManga?.episodeInfos.length }}</span>
      <n-divider vertical></n-divider>
      <span>已解锁：{{ selectedManga?.episodeInfos.filter(ep => !ep.isLocked).length }}</span>
      <n-divider vertical></n-divider>
      <span>已下载：{{ selectedManga?.episodeInfos.filter(ep => ep.isDownloaded).length }}</span>
      <n-divider vertical></n-divider>
      <span>已勾选：{{ checkedIds.length }}</span>
    </div>
    <div class="flex justify-between">
      左键拖动进行框选，右键打开菜单
      <n-button size="tiny" :disabled="selectedManga===undefined" @click="refreshEpisodes" class="w-1/6">刷新</n-button>
      <n-button size="tiny" :disabled="selectedManga===undefined" type="primary" @click="downloadEpisodes"
                class="w-1/4">
        下载勾选章节
      </n-button>
    </div>
    <n-empty v-if="selectedManga===undefined" description="请先进行漫画搜索">
    </n-empty>
    <SelectionArea v-else
                   ref="selectionAreaRef"
                   class="selection-container"
                   :options="{selectables: '.selectable', features: {deselectOnBlur: true}} as SelectionOptions"
                   @contextmenu="onContextMenu"
                   @move="onDragMove"
                   @start="onDragStart">
      <n-checkbox-group v-model:value="checkedIds" class="grid grid-cols-3 gap-1.5 w-full">
        <n-checkbox v-for="{episodeId, episodeTitle, isLocked, isDownloaded} in selectedManga.episodeInfos"
                    :key="episodeId"
                    :data-key="episodeId"
                    class="selectable hover:bg-gray-200!"
                    :value="episodeId"
                    :label="episodeTitle"
                    :disabled="isLocked || isDownloaded"
                    :class="{ selected: selectedIds.has(episodeId), downloaded: isDownloaded }"/>
      </n-checkbox-group>
    </SelectionArea>

    <n-dropdown
        placement="bottom-start"
        trigger="manual"
        :x="dropdownX"
        :y="dropdownY"
        :options="dropdownOptions"
        :show="showDropdown"
        :on-clickoutside="()=>showDropdown=false"
        @select="onDropdownSelect"
    />
  </div>
</template>

<style scoped>
.selection-container {
  @apply user-select-none overflow-auto;
}

.selection-container .selected {
  @apply bg-[rgb(204,232,255)];
}

.selection-container .downloaded {
  @apply bg-[rgba(24,160,88,0.16)];
}

:deep(.n-checkbox__label) {
  @apply overflow-hidden whitespace-nowrap text-ellipsis;
}

:global(.selection-area) {
  @apply bg-[rgba(46,115,252,0.5)];
}
</style>