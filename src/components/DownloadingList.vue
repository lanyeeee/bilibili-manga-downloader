<script setup lang="ts">

import {onMounted, ref} from "vue";
import {commands, Config, events} from "../bindings.ts";
import {NProgress, useNotification} from "naive-ui";
import {open} from "@tauri-apps/plugin-dialog";

type ProgressData = {
  title: string;
  current: number;
  total: number;
  percentage: number;
  indicator: string;
}

const notification = useNotification();

const config = defineModel<Config>("config", {required: true});

const progresses = ref<Map<number, ProgressData>>(new Map());
const overallProgress = ref<ProgressData>({title: "总进度", current: 0, total: 0, percentage: 0, indicator: ""});

onMounted(async () => {
  await events.downloadEpisodePendingEvent.listen(({payload}) => {
    let progressData: ProgressData = {
      title: `等待中 ${payload.title}`,
      current: 0,
      total: 0,
      percentage: 0,
      indicator: ""
    };
    progresses.value.set(payload.epId, progressData);
  });

  await events.downloadEpisodeStartEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.epId) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    progressData.total = payload.total;
    progressData.title = payload.title;
  });

  await events.downloadImageSuccessEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.epId) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    progressData.current += 1;
    progressData.percentage = Math.round(progressData.current / progressData.total * 100);
  });

  await events.downloadImageErrorEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.epId) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    notification.warning({
      title: "下载图片失败",
      description: payload.url,
      content: payload.errMsg,
      meta: progressData.title
    });
  });

  await events.downloadEpisodeEndEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.epId) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    if (payload.errMsg !== null) {
      notification.warning({title: "下载章节失败", content: payload.errMsg, meta: progressData.title});
    }
    progresses.value.delete(payload.epId);
  });

  await events.updateOverallDownloadProgressEvent.listen(({payload}) => {
    overallProgress.value.percentage = payload.percentage;
    overallProgress.value.current = payload.downloadedImageCount;
    overallProgress.value.total = payload.totalImageCount;
    console.log(payload);
  });

  await events.downloadSpeedEvent.listen(({payload}) => {
    overallProgress.value.indicator = payload.speed;
  });
});

async function showDownloadDirInFileManager() {
  if (config.value === undefined) {
    return;
  }
  const result = await commands.showPathInFileManager(config.value.downloadDir);
  if (result.status === "error") {
    notification.error({title: "打开下载目录失败", description: result.error});
  }
}

async function selectDownloadDir() {
  const selectedDirPath = await open({directory: true});
  if (selectedDirPath === null) {
    return;
  }
  config.value.downloadDir = selectedDirPath;
}

</script>

<template>
  <div class="flex flex-col gap-row-1">
    <n-h3 class="m-be-0">下载列表</n-h3>
    <div class="flex gap-col-1">
      <n-input v-model:value="config.downloadDir"
               :default-value="0"
               size="tiny"
               readonly
               placeholder="请选择漫画目录"
               @click="selectDownloadDir">
        <template #prefix>下载目录：</template>
      </n-input>
      <n-button size="tiny" @click="showDownloadDirInFileManager">打开下载目录</n-button>
    </div>
    <div class="grid grid-cols-[1fr_4fr_2fr]">
      <span class="text-ellipsis whitespace-nowrap overflow-hidden">{{ overallProgress.title }}</span>
      <n-progress :percentage="overallProgress.percentage" indicator-placement="inside" :height="21">
        {{ overallProgress.indicator }}
      </n-progress>
      <span>{{ overallProgress.current }}/{{ overallProgress.total }}</span>
    </div>
    <div class="grid grid-cols-[2fr_4fr_1fr]"
         v-for="[epId, {title, percentage, indicator}] in progresses"
         :key="epId">
      <span class="mb-1! text-ellipsis whitespace-nowrap overflow-hidden">{{ title }}</span>
      <n-progress class="" :percentage="percentage"/>
      <span>{{ indicator }}</span>
    </div>
  </div>
</template>