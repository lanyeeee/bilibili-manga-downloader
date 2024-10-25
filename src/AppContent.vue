<script setup lang="ts">
import {commands, Config, Manga} from "./bindings.ts";
import {ref, onMounted, watch} from "vue";
import {useNotification, useMessage} from "naive-ui";
import QrcodeViewer from "./components/QrcodeViewer.vue";
import DownloadingList from "./components/DownloadingList.vue";
import SearchPane from "./components/SearchPane.vue";
import EpisodePane from "./components/EpisodePane.vue";

const notification = useNotification();
const message = useMessage();

const config = ref<Config>();
const qrcodeViewerShowing = ref<boolean>(false);
const currentTabName = ref<"search" | "episode">("search");
const selectedManga = ref<Manga>();

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
  // 屏蔽浏览器右键菜单
  document.oncontextmenu = (event) => {
    event.preventDefault();
  };
  // 获取配置
  config.value = await commands.getConfig();
  // 如果没有buvid3，获取一个
  if (config.value.buvid3 === "") {
    const result = await commands.getBuvid3();
    if (result.status === "error") {
      notification.error({title: "获取buvid3失败", description: result.error});
      return;
    }
    config.value.buvid3 = result.data.buvid;
  }
});

async function test() {
  const result = await commands.getManga(26470);
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
    <div class="flex flex-1 overflow-hidden">
      <div class="basis-1/2 overflow-auto">
        <n-tabs v-model:value="currentTabName" type="line" size="small" class="h-full">
          <n-tab-pane class="h-full overflow-auto p-0!" name="search" tab="漫画搜索" display-directive="show">
            <search-pane v-model:current-tab-name="currentTabName" v-model:selected-manga="selectedManga"/>
          </n-tab-pane>
          <n-tab-pane class="h-full overflow-auto p-0!" name="episode" tab="章节详情" display-directive="show">
            <episode-pane v-model:selected-manga="selectedManga"/>
          </n-tab-pane>
        </n-tabs>
      </div>
      <div class="basis-1/2 overflow-auto">
        <downloading-list class="h-full" v-model:config="config"></downloading-list>
      </div>
    </div>
  </div>
  <n-modal preset="dialog" title="请使用BiliBili手机客户端扫描二维码登录" v-model:show="qrcodeViewerShowing">
    <qrcode-viewer v-if="config!==undefined" v-model:showing="qrcodeViewerShowing" v-model:config="config"/>
  </n-modal>
</template>
