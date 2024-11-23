<script setup lang="ts">

import {onMounted, ref} from "vue";
import {commands, Config, events} from "../bindings.ts";
import {useNotification} from "naive-ui";
import {open} from "@tauri-apps/plugin-dialog";
import SettingsDialog from "./SettingsDialog.vue";

type ProgressData = {
  comicTitle: string;
  episodeTitle: string;
  current: number;
  total: number;
  percentage: number;
  indicator: string;
}

const notification = useNotification();

const config = defineModel<Config>("config", {required: true});

const settingsDialogShowing = ref<boolean>(false);
const progresses = ref<Map<number, ProgressData>>(new Map());
const downloadSpeed = ref<string>("");

onMounted(async () => {
  await events.downloadPendingEvent.listen(({payload}) => {
    let progressData: ProgressData = {
      comicTitle: payload.comicTitle,
      episodeTitle: payload.episodeTitle,
      current: 0,
      total: 0,
      percentage: 0,
      indicator: ""
    };
    progresses.value.set(payload.id, progressData);
  });

  await events.downloadStartEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.id) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    progressData.total = payload.total;
  });

  await events.downloadImageSuccessEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.id) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    progressData.current = payload.current;
    progressData.percentage = Math.round(progressData.current / progressData.total * 100);
  });

  await events.downloadImageErrorEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.id) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    notification.warning({
      title: "下载图片失败",
      description: payload.url,
      content: payload.errMsg,
      meta: `${progressData.comicTitle} - ${progressData.episodeTitle}`
    });
  });

  await events.downloadEndEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.id) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    if (payload.errMsg !== null) {
      notification.warning({
        title: "下载章节失败",
        content: payload.errMsg,
        meta: `${progressData.comicTitle} - ${progressData.episodeTitle}`
      });
    }
    progresses.value.delete(payload.id);
  });

  await events.downloadSpeedEvent.listen(({payload}) => {
    downloadSpeed.value = payload.speed;
  });

  await events.setProxyErrorEvent.listen(({payload}) => {
    notification.error({title: "设置代理失败", description: payload.errMsg});
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
    <div class="flex gap-col-1">
      <n-input v-model:value="config.downloadDir"
               size="tiny"
               readonly
               placeholder="请选择漫画目录"
               @click="selectDownloadDir">
        <template #prefix>下载目录：</template>
      </n-input>
      <n-button size="tiny" @click="showDownloadDirInFileManager">下载目录</n-button>
      <n-button type="primary" secondary size="tiny" @click="settingsDialogShowing=true">更多设置</n-button>
    </div>
    <!--    <span>下载速度：{{ downloadSpeed }}</span>-->
    <div class="overflow-auto">
      <div class="grid grid-cols-[1fr_1fr_2fr]"
           v-for="[epId, { comicTitle, episodeTitle, percentage, total, current}] in progresses"
           :key="epId">
        <span class="mb-1! text-ellipsis whitespace-nowrap overflow-hidden">{{ comicTitle }}</span>
        <span class="mb-1! text-ellipsis whitespace-nowrap overflow-hidden">{{ episodeTitle }}</span>
        <span v-if="total===0">等待中</span>
        <n-progress v-else class="" :percentage="percentage">
          {{ current }}/{{ total }}
        </n-progress>
      </div>
    </div>
    <n-modal v-model:show="settingsDialogShowing">
      <settings-dialog v-model:showing="settingsDialogShowing" v-model:config="config"/>
    </n-modal>
  </div>
</template>