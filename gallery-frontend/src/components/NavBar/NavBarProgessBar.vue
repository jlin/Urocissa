<template>
  <v-toolbar flat height="2" class="no-select">
    <v-progress-linear
      v-if="!initializedStore.initialized"
      indeterminate
      color="primary"
      height="2"
      :style="{
        border: 0,
        backgroundColor: '#212121'
      }"
    >
    </v-progress-linear>
  </v-toolbar>
</template>
<script setup lang="ts">
import { getIsolationIdByRoute } from '@/script/common/functions';
import { useInitializedStore } from '@/store/initializedStore'
import { useRoute } from 'vue-router'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const initializedStore = useInitializedStore(isolationId)
</script>
