<script setup lang="ts">

import AppContent from "./AppContent.vue";
</script>

<template>
  <n-modal-provider>
    <n-notification-provider placement="bottom-right">
      <n-message-provider>
        <app-content/>
      </n-message-provider>
    </n-notification-provider>
  </n-modal-provider>
</template>
