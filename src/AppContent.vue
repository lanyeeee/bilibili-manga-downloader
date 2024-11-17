<script setup lang="ts">
import {Comic, commands, Config, UserProfileRespData} from "./bindings.ts";
import {onMounted, ref, watch} from "vue";
import {useMessage, useNotification} from "naive-ui";
import QrcodeViewer from "./components/QrcodeViewer.vue";
import DownloadingList from "./components/DownloadingList.vue";
import SearchPane from "./components/SearchPane.vue";
import EpisodePane from "./components/EpisodePane.vue";
import CookieLoginDialog from "./components/CookieLoginDialog.vue";

const notification = useNotification();
const message = useMessage();

const config = ref<Config>();
const qrcodeViewerShowing = ref<boolean>(false);
const cookieLoginDialogShowing = ref<boolean>(false);
const currentTabName = ref<"search" | "episode">("search");
const selectedComic = ref<Comic>();
const userProfile = ref<UserProfileRespData>();

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

watch(() => config.value?.accessToken, async () => {
  if (config.value === undefined || config.value.accessToken === "") {
    return;
  }
  const result = await commands.getUserProfile();
  if (result.status === "error") {
    notification.error({title: "获取用户信息失败", description: result.error});
    userProfile.value = undefined;
    return;
  }
  userProfile.value = result.data;
  message.success("获取用户信息成功");
});

onMounted(async () => {
  // 屏蔽浏览器右键菜单
  document.oncontextmenu = (event) => {
    event.preventDefault();
  };
  // 获取配置
  config.value = await commands.getConfig();
});

async function test() {
  notification.error({
    title: "标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 标题 ",
    description: "描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 描述 ",
    content: "内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 内容 ",
  });

}

</script>

<template>
  <div v-if="config!==undefined" class="h-screen flex flex-col">
    <div class="flex flex-1 overflow-hidden">
      <div class="basis-1/2 overflow-auto">
        <n-tabs v-model:value="currentTabName" type="line" size="small" class="h-full">
          <n-tab-pane class="h-full overflow-auto p-0!" name="search" tab="漫画搜索" display-directive="show">
            <search-pane v-model:current-tab-name="currentTabName" v-model:selected-comic="selectedComic"/>
          </n-tab-pane>
          <n-tab-pane class="h-full overflow-auto p-0!" name="episode" tab="章节详情" display-directive="show">
            <episode-pane v-model:selected-comic="selectedComic"/>
          </n-tab-pane>
        </n-tabs>
      </div>
      <div class="basis-1/2 flex flex-col overflow-hidden">
        <div class="flex">
          <n-button @click="qrcodeViewerShowing=true" type="primary">二维码登录</n-button>
          <n-button @click="cookieLoginDialogShowing=true" type="primary" secondary>Cookie登录</n-button>
          <n-button @click="test">测试用</n-button>
          <div v-if="userProfile!==undefined" class="flex flex-justify-end">
            <n-avatar round
                      :img-props="{referrerpolicy: 'no-referrer'}"
                      :size="32"
                      :src="userProfile.face"/>
            <span class="whitespace-nowrap">{{ userProfile.name }}</span>
          </div>
        </div>
        <downloading-list class="h-full" v-model:config="config"></downloading-list>
      </div>
    </div>
  </div>
  <n-modal preset="dialog" title="请使用BiliBili手机客户端扫描二维码登录" v-model:show="qrcodeViewerShowing">
    <qrcode-viewer v-if="config!==undefined" v-model:showing="qrcodeViewerShowing" v-model:config="config"/>
  </n-modal>
  <n-modal v-model:show="cookieLoginDialogShowing">
    <cookie-login-dialog v-if="config!==undefined" v-model:showing="cookieLoginDialogShowing" v-model:config="config"/>
  </n-modal>
</template>
