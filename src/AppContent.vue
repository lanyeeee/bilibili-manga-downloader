<script setup lang="ts">
import {commands, Config} from "./bindings.ts";
import {ref, onMounted, watch} from "vue";
import {useNotification, useMessage} from "naive-ui";
import QrcodeViewer from "./components/QrcodeViewer.vue";

const notification = useNotification();
const message = useMessage();

const config = ref<Config>();
const qrcodeViewerShowing = ref<boolean>(false);

watch(config, async () => {
  if (config.value === undefined) {
    return;
  }
  const result = await commands.saveConfig(config.value);
  if (result.status === "error") {
    notification.error({title: "保存配置失败", description: result.error});
    return;
  }

  message.success("保存配置成功");
}, {deep: true});

onMounted(async () => {
  // 获取配置
  config.value = await commands.getConfig();
});

async function test() {
  const result = await commands.searchManga("一拳超人");
  console.log(result);
}

</script>

<template>
  <div v-if="config!==undefined" class="h-screen flex flex-col">
    <div class="flex">
      <n-input v-model:value="config.sessdata" placeholder="" clearable>
        <template #prefix>
          SESSDATA:
        </template>
      </n-input>
      <n-button @click="qrcodeViewerShowing=true">二维码登录</n-button>
      <!--   TODO: 检测SESSDATA是否有效的按钮   -->
      <n-button @click="test">测试用</n-button>
    </div>
  </div>
  <n-modal preset="dialog" title="请使用BiliBili手机客户端扫描二维码登录" v-model:show="qrcodeViewerShowing">
    <qrcode-viewer v-if="config!==undefined" v-model:showing="qrcodeViewerShowing" v-model:config="config"/>
  </n-modal>
</template>
