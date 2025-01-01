<template>
  <v-list-item prepend-icon="mdi-image-refresh-outline" @click="regeneratePreview">
    <v-list-item-title class="wrap">Regenerate Preview</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { usePrefetchStore } from '@/store/prefetchStore'
import axios from 'axios'
import { getIsolationIdByRoute } from '@/script/common/functions'
import { useMessageStore } from '@/store/messageStore'

const props = defineProps<{
  indexList: number[]
}>()

const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const prefetchStore = usePrefetchStore(isolationId)
const messageStore = useMessageStore('mainId')
const regeneratePreview = async () => {
  const indexArray = props.indexList
  const regenerateData = {
    indexArray: indexArray,
    timestamp: prefetchStore.timestamp
  }
  try {
    const response = await axios.put('/put/regenerate-preview', regenerateData, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    console.log('Response:', response.data)
    messageStore.message = 'Regenerating preview...'
    messageStore.warn = false
    messageStore.showMessage = true
  } catch (error) {
    console.error('Error:', error)
  }
}
</script>
