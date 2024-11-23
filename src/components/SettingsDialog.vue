<script setup lang="ts">
import {commands, Config} from "../bindings.ts";
import {computed, ref} from "vue";
import {path} from "@tauri-apps/api";
import {appDataDir} from "@tauri-apps/api/path";
import {useNotification} from "naive-ui";

const notification = useNotification();

const config = defineModel<Config>("config", {required: true});
const showing = defineModel<boolean>("showing", {required: true});
const proxyHost = ref<string>(config.value.proxyHost);
const disableProxyHostAndPort = computed(() => config.value.proxyMode !== "Custom");

async function showConfigInFileManager() {
  const configName = "config.json";
  const configPath = await path.join(await appDataDir(), configName);
  const result = await commands.showPathInFileManager(configPath);
  if (result.status === "error") {
    notification.error({title: "打开配置文件失败", description: result.error});
  }
}
</script>

<template>
  <n-dialog :showIcon="false"
            title="设置"
            content-style=""
            @close="showing=false">
    <div class="flex flex-col gap-row-2">
      <n-radio-group v-model:value="config.archiveFormat">
        下载格式：
        <n-radio value="Image">文件夹-图片</n-radio>
        <n-radio value="Zip">zip</n-radio>
        <n-radio value="Cbz">cbz</n-radio>
      </n-radio-group>
      <n-radio-group v-model:value="config.proxyMode">
        代理类型：
        <n-radio value="NoProxy">直连</n-radio>
        <n-radio value="System">系统代理</n-radio>
        <n-radio value="Custom">自定义</n-radio>
      </n-radio-group>
      <div class="flex">
        <n-input :disabled=disableProxyHostAndPort
                 v-model:value="proxyHost"
                 size="tiny"
                 placeholder=""
                 @blur="config.proxyHost=proxyHost"
                 @keydown.enter="config.proxyHost=proxyHost">
          <template #prefix>
            主机:
          </template>
        </n-input>
        <n-input-number :disabled=disableProxyHostAndPort
                        v-model:value="config.proxyPort"
                        size="tiny"
                        placeholder=""
                        :parse="(x:string) => parseInt(x)">
          <template #prefix>
            端口:
          </template>
        </n-input-number>
      </div>
      <n-button size="tiny" @click="showConfigInFileManager">打开配置文件目录</n-button>
    </div>
  </n-dialog>

</template>