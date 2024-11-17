<script setup lang="ts">
import {ref} from "vue";
import {AppQrcodeData, AppQrcodeStatus, commands, Config} from "../bindings.ts";
import {useMessage, useNotification} from "naive-ui";

const notification = useNotification();
const message = useMessage();

const showing = defineModel<boolean>("showing", {required: true});
const config = defineModel<Config>("config", {required: true});

const sessdata = ref<string>("");
const csrf = ref<string>("");

async function onLogin() {
  const appQrcodeData = await generateAppQrcode();
  if (appQrcodeData === undefined) {
    return;
  }
  console.log(appQrcodeData.auth_code);

  // 确认生成的App端二维码
  const confirmed = await confirmAppQrcode(appQrcodeData);
  if (!confirmed) {
    return;
  }
  // 确认App端的二维码成功，轮询App端的二维码状态，以获取access_token
  while (showing.value) {
    const appQrcodeStatus = await getAppQrcodeStatus(appQrcodeData);
    if (appQrcodeStatus === undefined) {
      return;
    }

    await handleAppQrcodeStatus(appQrcodeStatus);
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
}

async function generateAppQrcode(): Promise<AppQrcodeData | undefined> {
  const result = await commands.generateAppQrcode();
  if (result.status === "error") {
    notification.error({title: "生成App二维码失败", description: result.error});
    return;
  }

  return result.data;
}

async function confirmAppQrcode(appQrcodeData: AppQrcodeData): Promise<boolean> {
  if (sessdata.value === "") {
    message.error("请输入SESSDATA");
    return false;
  }
  if (csrf.value === "") {
    message.error("请输入bili_jct");
    return false;
  }
  const result = await commands.confirmAppQrcode(appQrcodeData.auth_code, sessdata.value, csrf.value);
  if (result.status === "error") {
    notification.error({title: "确认App二维码失败", description: result.error});
    return false;
  }

  const confirmResult = result.data;
  if (confirmResult.code !== 0) {
    notification.error({
      title: "确认App二维码失败，code不为0",
      description: JSON.stringify(confirmResult),
      content: confirmResult.code === -101 ? "请在浏览器里退出账号后重新登录，用新生成的SESSDATA和bili_jct重试" : undefined,
    });
    return false;
  }

  return true;
}

async function getAppQrcodeStatus(appQrcodeData: AppQrcodeData): Promise<AppQrcodeStatus | undefined> {
  const result = await commands.getAppQrcodeStatus(appQrcodeData.auth_code);
  if (result.status === "error") {
    notification.error({title: "获取App二维码状态失败", description: result.error});
    return;
  }

  return result.data;
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

</script>

<template>
  <n-dialog :showIcon="false"
            title="Cookie登录"
            positive-text="登录"
            @positive-click="onLogin"
            @close="showing=false"
            @keydown.enter="onLogin">
    <div class="flex flex-col gap-row-2">
      <n-input v-model:value="sessdata" placeholder="">
        <template #prefix>
          SESSDATA:
        </template>
      </n-input>
      <n-input v-model:value="csrf" placeholder="">
        <template #prefix>
          bili_jct:
        </template>
      </n-input>
    </div>
  </n-dialog>
</template>