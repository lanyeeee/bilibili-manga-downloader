<script setup lang="ts">
import {SelectionArea, SelectionEvent, SelectionOptions} from "@viselect/vue";
import {nextTick, ref, watch} from "vue";
import {Comic, commands} from "../bindings.ts";
import {useNotification} from "naive-ui";

const notification = useNotification();

const selectedComic = defineModel<Comic | undefined>("selectedComic", {required: true});

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

watch(selectedComic, () => {
  checkedIds.value = [];
  selectedIds.value.clear();
  selectionAreaRef.value?.selection?.clearSelection();
});

function extractIds(elements: Element[]): number[] {
  return elements.map(element => element.getAttribute("data-key"))
      .filter(Boolean)
      .map(Number)
      .filter(id => {
        const episodeInfo = selectedComic.value?.episodeInfos.find(ep => ep.episodeId === id);
        if (episodeInfo !== undefined) {
          return !episodeInfo.isLocked && !episodeInfo.isDownloaded;
        }

        const albumPlusDetail = selectedComic.value?.albumPlus.list.find(detail => detail.item.id === id);
        if (albumPlusDetail !== undefined) {
          return !albumPlusDetail.isLock && !albumPlusDetail.isDownloaded;
        }

        return false;
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
    selectedComic.value?.episodeInfos
        .filter(ep => !ep.isLocked && !ep.isDownloaded && !checkedIds.value.includes(ep.episodeId))
        .forEach(ep => checkedIds.value.push(ep.episodeId));

    selectedComic.value?.albumPlus.list
        .filter(detail => !detail.isLock && !detail.isDownloaded && !checkedIds.value.includes(detail.item.id))
        .forEach(detail => checkedIds.value.push(detail.item.id));
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

async function downloadChecked() {
  const episodesToDownload = selectedComic.value?.episodeInfos
      .filter(ep => !ep.isDownloaded && checkedIds.value.includes(ep.episodeId));
  if (episodesToDownload !== undefined) {
    const result = await commands.downloadEpisodes(episodesToDownload);
    if (result.status === "error") {
      notification.error({title: "下载章节失败", description: result.error});
    }

    for (const downloadedEp of episodesToDownload) {
      const episode = selectedComic.value?.episodeInfos.find(ep => ep.episodeId === downloadedEp.episodeId);
      if (episode !== undefined) {
        episode.isDownloaded = true;
        checkedIds.value = checkedIds.value.filter(id => id !== downloadedEp.episodeId);
      }
    }
  }

  const albumPlusItemToDownload = selectedComic.value?.albumPlus.list
      .filter(detail => !detail.isDownloaded && checkedIds.value.includes(detail.item.id))
      .map(detail => detail.item);
  if (albumPlusItemToDownload !== undefined) {
    const result = await commands.downloadAlbumPlusItems(albumPlusItemToDownload);
    if (result.status === "error") {
      notification.error({title: "下载特典失败", description: result.error});
    }

    for (const downloadedItem of albumPlusItemToDownload) {
      const detail = selectedComic.value?.albumPlus.list.find(detail => detail.item.id === downloadedItem.id);
      if (detail !== undefined) {
        detail.isDownloaded = true;
        checkedIds.value = checkedIds.value.filter(id => id !== downloadedItem.id);
      }
    }
  }


}

async function refreshEpisodes() {
  if (selectedComic.value === undefined) {
    return;
  }
  const result = await commands.getComic(selectedComic.value.id);
  if (result.status === "error") {
    notification.error({title: "获取漫画章节详情失败", description: result.error});
    return;
  }
  selectedComic.value = result.data;
}

</script>

<template>
  <div class="h-full flex flex-col">
    <div class="flex flex-justify-around">
      <!--   TODO: 还要把特典算在内   -->
      <span>总章数：{{ selectedComic?.episodeInfos.length }}</span>
      <n-divider vertical></n-divider>
      <span>已解锁：{{ selectedComic?.episodeInfos.filter(ep => !ep.isLocked).length }}</span>
      <n-divider vertical></n-divider>
      <span>已下载：{{ selectedComic?.episodeInfos.filter(ep => ep.isDownloaded).length }}</span>
      <n-divider vertical></n-divider>
      <span>已勾选：{{ checkedIds.length }}</span>
    </div>
    <div class="flex justify-between">
      左键拖动进行框选，右键打开菜单
      <n-button size="tiny" :disabled="selectedComic===undefined" @click="refreshEpisodes" class="w-1/6">刷新</n-button>
      <n-button size="tiny" :disabled="selectedComic===undefined" type="primary" @click="downloadChecked"
                class="w-1/4">
        下载勾选项
      </n-button>
    </div>
    <n-empty v-if="selectedComic===undefined" description="请先进行漫画搜索"/>
    <div v-else class="flex-1 flex flex-col overflow-auto">
      <n-divider class="my-0!" title-placement="left">
        正文
      </n-divider>
      <SelectionArea ref="selectionAreaRef"
                     class="selection-container flex-grow"
                     :options="{selectables: '.selectable', features: {deselectOnBlur: true}} as SelectionOptions"
                     @contextmenu="onContextMenu"
                     @move="onDragMove"
                     @start="onDragStart">
        <n-checkbox-group v-model:value="checkedIds" class="flex flex-col">
          <div class="grid grid-cols-3 gap-1.5 w-full">
            <n-checkbox v-for="{episodeId, episodeTitle, isLocked, isDownloaded} in selectedComic.episodeInfos"
                        :key="episodeId"
                        :data-key="episodeId"
                        class="selectable hover:bg-gray-200!"
                        :value="episodeId"
                        :label="episodeTitle"
                        :disabled="isLocked || isDownloaded"
                        :class="{ selected: selectedIds.has(episodeId), downloaded: isDownloaded }"/>
          </div>
        </n-checkbox-group>
      </SelectionArea>
      <n-divider class="my-0!" title-placement="left">
        特典
      </n-divider>
      <SelectionArea ref="selectionAreaRef"
                     class="selection-container flex-shrink-0 mb-3 max-h-[30%]"
                     :options="{selectables: '.selectable', features: {deselectOnBlur: true}} as SelectionOptions"
                     @contextmenu="onContextMenu"
                     @move="onDragMove"
                     @start="onDragStart">
        <n-checkbox-group v-model:value="checkedIds" class="flex flex-col">
          <div class="grid grid-cols-3 gap-1.5 w-full">
            <n-checkbox v-for="{isLock, isDownloaded, item} in selectedComic.albumPlus.list"
                        :key="item.id"
                        :data-key="item.id"
                        class="selectable hover:bg-gray-200!"
                        :value="item.id"
                        :label="item.title"
                        :disabled="isLock || isDownloaded"
                        :class="{ selected: selectedIds.has(item.id), downloaded: isDownloaded }"/>
          </div>
        </n-checkbox-group>
      </SelectionArea>
    </div>


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