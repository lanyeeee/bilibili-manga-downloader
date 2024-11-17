<script setup lang="ts">

import {AppQrcodeData, AppQrcodeStatus, commands, Config, WebQrcodeData, WebQrcodeStatusRespData} from "../bindings.ts";
import {ref, watch} from "vue";
import {useMessage, useNotification} from "naive-ui";

const message = useMessage();
const notification = useNotification();

const showing = defineModel<boolean>("showing", {required: true});
const config = defineModel<Config>("config", {required: true});

const imgRef = ref<HTMLImageElement>();
const qrcodeStatusMessage = ref<string>();


watch(showing, async () => {
  if (!showing.value) {
    return;
  }
  // 生成web端和app端二维码
  const generateResult = await generateQrcode();
  if (generateResult === undefined) {
    return;
  }

  const [webQrcodeData, appQrcodeData] = generateResult;
  // 每隔一秒获取一次Web二维码状态，并处理，直到showing为false
  while (showing.value) {
    const webQrcodeStatus = await getWebQrcodeStatus(webQrcodeData);
    if (webQrcodeStatus === undefined) {
      return;
    }

    await handleWebQrcodeStatus(webQrcodeStatus, appQrcodeData);
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
}, {immediate: true});

async function generateQrcode(): Promise<[WebQrcodeData, AppQrcodeData] | undefined> {
  // 并发生成web和app二维码
  const [webQrcodeData, appQrcodeData] = await Promise.all([
    generateWebQrcode(),
    generateAppQrcode(),
  ]);

  if (webQrcodeData === undefined || appQrcodeData === undefined) {
    return;
  }

  return [webQrcodeData, appQrcodeData];
}

async function generateWebQrcode(): Promise<WebQrcodeData | undefined> {
  const result = await commands.generateWebQrcode();
  if (result.status === "error") {
    notification.error({title: "生成Web二维码失败", description: result.error});
    return;
  }

  const webQrcodeData = result.data;
  if (imgRef.value === undefined) {
    return;
  }

  imgRef.value.src = `data:image/jpeg;base64,${webQrcodeData.base64}`;
  return webQrcodeData;
}

async function generateAppQrcode(): Promise<AppQrcodeData | undefined> {
  const result = await commands.generateAppQrcode();
  if (result.status === "error") {
    notification.error({title: "生成App二维码失败", description: result.error});
    return;
  }

  return result.data;
}

async function getWebQrcodeStatus(webQrcodeData: WebQrcodeData): Promise<WebQrcodeStatusRespData | undefined> {
  const result = await commands.getWebQrcodeStatus(webQrcodeData.qrcodeKey);
  if (result.status === "error") {
    notification.error({title: "获取Web二维码状态失败", description: result.error});
    return;
  }

  return result.data;
}

async function getAppQrcodeStatus(appQrcodeData: AppQrcodeData): Promise<AppQrcodeStatus | undefined> {
  const result = await commands.getAppQrcodeStatus(appQrcodeData.auth_code);
  if (result.status === "error") {
    notification.error({title: "获取App二维码状态失败", description: result.error});
    return;
  }

  return result.data;
}

async function confirmAppQrcode(authCode: string, sessdata: string, csrf: string): Promise<boolean> {
  const result = await commands.confirmAppQrcode(authCode, sessdata, csrf);
  if (result.status === "error") {
    notification.error({title: "确认App二维码失败", description: result.error});
    return false;
  }

  const confirmResult = result.data;
  if (confirmResult.code !== 0) {
    notification.error({
      title: "确认App二维码失败",
      description: JSON.stringify(confirmResult)
    });
    return false;
  }

  return true;
}

async function handleWebQrcodeStatus(webQrcodeStatus: WebQrcodeStatusRespData, appQrcodeData: AppQrcodeData) {
  // 更新页面上显示的二维码状态
  qrcodeStatusMessage.value = webQrcodeStatus.message;
  // 处理二维码状态
  const code = webQrcodeStatus.code;
  if (![0, 86101, 86090, 86038].includes(code)) {
    notification.error({
      title: "处理Web二维码状态失败，预料之外的code",
      description: JSON.stringify(webQrcodeStatus),
    });
    return;
  }
  // 不是登录成功状态就返回
  if (code !== 0) {
    return;
  }
  // 登录成功，准备用Web端的SESSDATA去确认App端的二维码，以换取access_token
  const sessdataAndCsrf = getSessdataAndCsrf(webQrcodeStatus);
  if (sessdataAndCsrf === undefined) {
    return;
  }
  const [sessdata, csrf] = sessdataAndCsrf;
  const authCode = appQrcodeData.auth_code;
  // 确认App端的二维码
  const confirmed = await confirmAppQrcode(authCode, sessdata, csrf);
  if (!confirmed) {
    return;
  }
  // 用Web端的SESSDATA和csrf确认App端的二维码成功，轮询App端的二维码状态，以获取access_token
  qrcodeStatusMessage.value = "正在获取access_token";
  while (showing.value) {
    const appQrcodeStatus = await getAppQrcodeStatus(appQrcodeData);
    if (appQrcodeStatus === undefined) {
      return;
    }

    await handleAppQrcodeStatus(appQrcodeStatus);
    await new Promise(resolve => setTimeout(resolve, 1000));
  }

}

async function handleAppQrcodeStatus(appQrcodeStatus: AppQrcodeStatus) {
  const code = appQrcodeStatus.code;
  if (![0, 86038, 86039, 86090].includes(code)) {
    notification.error({
      title: "处理App二维码状态失败，预料之外的code",
      description: JSON.stringify(appQrcodeStatus),
    });
    return;
  }
  // 不是登录成功状态就返回，继续轮询
  if (code !== 0) {
    return;
  }
  // 登录成功，把access_token保存到config中
  config.value.accessToken = appQrcodeStatus.access_token;
  showing.value = false;
  message.success("登录成功");
}

function getSessdataAndCsrf(webQrcodeStatus: WebQrcodeStatusRespData): [string, string] | undefined {
  const params = new URLSearchParams(webQrcodeStatus.url);

  const sessdata = params.get("SESSDATA");
  if (sessdata === null) {
    notification.error({
      title: "未在Web二维码登录成功后的返回值中找到SESSDATA",
      description: JSON.stringify(webQrcodeStatus),
    });
    return;
  }

  const csrf = params.get("bili_jct");
  if (csrf === null) {
    notification.error({
      title: "未在Web二维码登录成功后的返回值中找到bili_jct",
      description: JSON.stringify(webQrcodeStatus),
    });
    return;
  }

  return [sessdata, csrf];
}

</script>

<template>
  <div class="flex flex-col">
    二维码状态：{{ qrcodeStatusMessage }}
    <img ref="imgRef" src="" alt="">
  </div>
</template>

<style scoped>

</style>