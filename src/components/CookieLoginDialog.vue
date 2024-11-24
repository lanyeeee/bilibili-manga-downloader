<script setup lang="ts">
import {Config} from "../bindings.ts";

const showing = defineModel<boolean>("showing", {required: true});
const config = defineModel<Config>("config", {required: true});

</script>

<template>
  <n-dialog :showIcon="false"
            title="Cookie登录"
            @close="showing=false">
    <div class="flex flex-col gap-row-2">
      <n-input v-model:value="config.cookie" placeholder="请将浏览器中的Cookie完整的粘贴至此">
        <template #prefix>
          Cookie:
        </template>
      </n-input>
    </div>
  </n-dialog>
</template>