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
  if (result.data.mid !== config.value.uid) {
    config.value.uid = result.data.mid;
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
  // 检查更新
  const ts = Date.now();
  // 如果上次检查更新时间距离现在超过1分钟，就检查更新
  if (ts - config.value.lastUpdateCheckTs > 60 * 1000) {
    await checkUpdate();
    config.value.lastUpdateCheckTs = ts;
  }
});

async function checkUpdate() {
  const result = await commands.checkUpdate();
  if (result.status === "error") {
    notification.error({
      title: "检查更新失败",
      description: result.error,
      content: "如果继续使用，可能会因为漏掉重要更新(例如风控策略变更)而导致封号"
    });
    return;
  }
  const checkUpdateResult = result.data;
  if (checkUpdateResult.importantVersions.length > 0) {
    // 重复发3次
    const versions = checkUpdateResult.importantVersions.join(", ");
    for (let i = 0; i < 3; i++) {
      notification.error({
        title: "有重要更新，请立刻停止使用当前版本",
        description: "请前往 https://github.com/lanyeeee/bilibili-manga-downloader 查看Release页面",
        content: `重要更新版本: ${versions}`,
        meta: "很重要所以说3遍"
      });
    }
    return;
  }

  if (checkUpdateResult.normalVersions.length > 0) {
    const versions = checkUpdateResult.normalVersions.join(", ");
    notification.info({
      title: "有可选更新",
      description: "请前往 https://github.com/lanyeeee/bilibili-manga-downloader 的Release页面下载",
      content: `可选更新版本: ${versions}`,
      meta: "如果当前使用没有问题，可以不更新"
    });
    return;
  }

  notification.success({title: "当前版本是最新版本", duration: 2000});
}

async function test() {

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
      <div class="basis-1/2 flex flex-col overflow-hidden h-full">
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
        <downloading-list class="overflow-auto" v-model:config="config"></downloading-list>
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
