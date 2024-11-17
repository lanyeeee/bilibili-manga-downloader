<script setup lang="ts">

import {onMounted, ref} from "vue";
import {commands, Config, events} from "../bindings.ts";
import {NProgress, useNotification} from "naive-ui";
import {open} from "@tauri-apps/plugin-dialog";
import {path} from "@tauri-apps/api";
import {appDataDir} from "@tauri-apps/api/path";

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
const downloadSpeed = ref<string>("");

onMounted(async () => {
  await events.downloadPendingEvent.listen(({payload}) => {
    let progressData: ProgressData = {
      title: `等待中 ${payload.title}`,
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
    progressData.title = payload.title;
  });

  await events.downloadImageSuccessEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.id) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    progressData.current += 1;
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
      meta: progressData.title
    });
  });

  await events.downloadEndEvent.listen(({payload}) => {
    const progressData = progresses.value.get(payload.id) as (ProgressData | undefined);
    if (progressData === undefined) {
      return;
    }
    if (payload.errMsg !== null) {
      notification.warning({title: "下载章节失败", content: payload.errMsg, meta: progressData.title});
    }
    progresses.value.delete(payload.id);
  });

  await events.downloadSpeedEvent.listen(({payload}) => {
    downloadSpeed.value = payload.speed;
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

async function showConfigInFileManager() {
  const configName = "config.json";
  const configPath = await path.join(await appDataDir(), configName);
  const result = await commands.showPathInFileManager(configPath);
  if (result.status === "error") {
    notification.error({title: "打开配置文件失败", description: result.error});
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
      <n-button size="tiny" @click="showConfigInFileManager">配置目录</n-button>
    </div>
    <n-radio-group v-model:value="config.archiveFormat">
      下载格式：
      <n-radio value="Image">文件夹-图片</n-radio>
      <n-radio value="Zip">zip</n-radio>
      <n-radio value="Cbz">cbz</n-radio>
    </n-radio-group>
    <span>下载速度：{{ downloadSpeed }}</span>
    <div class="flex-1 overflow-auto">
      <div class="grid grid-cols-[2fr_4fr_1fr]"
           v-for="[epId, {title, percentage, indicator}] in progresses"
           :key="epId">
        <span class="mb-1! text-ellipsis whitespace-nowrap overflow-hidden">{{ title }}</span>
        <n-progress class="" :percentage="percentage"/>
        <span>{{ indicator }}</span>
      </div>
    </div>
  </div>
</template>