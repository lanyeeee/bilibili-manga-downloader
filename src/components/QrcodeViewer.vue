<script setup lang="ts">

import {commands, Config, QrcodeData, QrcodeStatusRespData} from "../bindings.ts";
import {ref, watch} from "vue";
import {useMessage, useNotification} from "naive-ui";

const message = useMessage();
const notification = useNotification();

const showing = defineModel<boolean>("showing", {required: true});
const config = defineModel<Config>("config", {required: true});

const qrcodeData = ref<QrcodeData>();
const imgRef = ref<HTMLImageElement>();
const qrcodeStatus = ref<QrcodeStatusRespData>();


watch(showing, async () => {
  if (showing.value) {
    await generateQrcode();
  } else {
    qrcodeData.value = undefined;
  }
}, {immediate: true});


async function generateQrcode() {
  const result = await commands.generateQrcode();
  if (result.status === "error") {
    notification.error({title: "获取二维码失败", description: result.error});
    return;
  }
  qrcodeData.value = result.data;
  if (imgRef.value === undefined) {
    return;
  }
  imgRef.value.src = `data:image/jpeg;base64,${qrcodeData.value.base64}`;
  // 每隔一秒获取一次二维码状态，直到showing为false
  const interval = setInterval(async () => {
    if (!showing.value) {
      clearInterval(interval);
      return;
    }
    await getQrcodeStatus();
    handleQrcodeStatus();
  }, 1000);
}

async function getQrcodeStatus() {
  if (qrcodeData.value === undefined) {
    return;
  }
  const result = await commands.getQrcodeStatus(qrcodeData.value?.qrcodeKey);
  if (result.status === "error") {
    notification.error({title: "获取二维码状态失败", description: result.error});
    return;
  }
  qrcodeStatus.value = result.data;
  console.log(qrcodeStatus.value);
}

function handleQrcodeStatus() {
  if (qrcodeStatus.value === undefined) {
    return;
  }

  const code = qrcodeStatus.value.code;
  if (![0, 86101, 86090, 86038].includes(code)) {
    notification.error({
      title: "处理二维码状态失败，预料之外的code",
      description: JSON.stringify(qrcodeStatus.value),
    });
    return;
  }

  if (code === 0) {
    config.value.sessdata = qrcodeStatus.value.url.split("SESSDATA=")[1].split("&")[0];
    showing.value = false;
    message.success("登录成功");
  }

}

</script>

<template>
  <div class="flex flex-col">
    二维码状态：{{ qrcodeStatus?.message }}
    <img ref="imgRef" src="" alt="">
  </div>
</template>

<style scoped>

</style>