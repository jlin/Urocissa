<template>
  <v-list-item prepend-icon="mdi-image-refresh-outline" @click="regeneratePreview">
    <v-list-item-title class="wrap">Regenerate Preview</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import axios from 'axios'
import { getIsolationIdByRoute } from '@/script/common/functions'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)
const prefetchStore = usePrefetchStore(isolationId)
const regeneratePreview = async () => {
  const indexArray = Array.from(collectionStore.editModeCollection)
  const regenerateData = {
    indexArray: indexArray,
    timestamp: prefetchStore.timestamp
  }
  try {
    const response = await axios.post('/put/regenerate-preview', regenerateData, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    console.log('Response:', response.data)
  } catch (error) {
    console.error('Error:', error)
  }
}
</script>
