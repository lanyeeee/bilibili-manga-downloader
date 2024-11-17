<script setup lang="ts">

import {AppQrcodeData, AppQrcodeStatus, commands, Config} from "../bindings.ts";
import {ref, watch} from "vue";
import {useMessage, useNotification} from "naive-ui";

const message = useMessage();
const notification = useNotification();

const showing = defineModel<boolean>("showing", {required: true});
const config = defineModel<Config>("config", {required: true});

const appQrcodeData = ref<AppQrcodeData>();
const imgRef = ref<HTMLImageElement>();
const appQrcodeStatus = ref<AppQrcodeStatus>();


watch(showing, async () => {
  if (showing.value) {
    await generateAppQrcode();
  } else {
    appQrcodeData.value = undefined;
  }
}, {immediate: true});


async function generateAppQrcode() {
  const result = await commands.generateAppQrcode();
  if (result.status === "error") {
    notification.error({title: "获取二维码失败", description: result.error});
    return;
  }
  appQrcodeData.value = result.data;
  if (imgRef.value === undefined) {
    return;
  }
  imgRef.value.src = `data:image/jpeg;base64,${appQrcodeData.value.base64}`;
  // 每隔一秒获取一次二维码状态，直到showing为false
  const interval = setInterval(async () => {
    if (!showing.value) {
      clearInterval(interval);
      return;
    }
    await getAppQrcodeStatus();
    handleAppQrcodeStatus();
  }, 1000);
}

async function getAppQrcodeStatus() {
  if (appQrcodeData.value === undefined) {
    return;
  }
  const result = await commands.getAppQrcodeStatus(appQrcodeData.value?.auth_code);
  if (result.status === "error") {
    notification.error({title: "获取二维码状态失败", description: result.error});
    return;
  }
  appQrcodeStatus.value = result.data;
  console.log(appQrcodeStatus.value);
}

function handleAppQrcodeStatus() {
  if (appQrcodeStatus.value === undefined) {
    return;
  }

  const code = appQrcodeStatus.value.code;
  if (![0, 86038, 86039, 86090].includes(code)) {
    notification.error({
      title: "处理二维码状态失败，预料之外的code",
      description: JSON.stringify(appQrcodeStatus.value),
    });
    return;
  }

  if (code === 0) {
    config.value.accessToken = appQrcodeStatus.value.access_token;
    config.value.sessdata = appQrcodeStatus.value.cookie_info.cookies.find(c => c.name === "SESSDATA")?.value ?? "";
    showing.value = false;
    message.success("登录成功");
  }

}

</script>

<template>
  <div class="flex flex-col">
    二维码状态：{{ appQrcodeStatus?.message }}
    <img ref="imgRef" src="" alt="">
  </div>
</template>

<style scoped>

</style>