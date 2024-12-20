<template>
  <v-list-item prepend-icon="mdi-image-refresh-outline" @click="regenerateMetadata">
    <v-list-item-title class="wrap">Regenerate Metadata</v-list-item-title>
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

const regenerateMetadata = async () => {
  const indexArray = props.indexList
  const regenerateData = {
    indexArray: indexArray,
    timestamp: prefetchStore.timestamp
  }
  try {
    const response = await axios.post('/put/regenerate-metadata', regenerateData, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    console.log('Response:', response.data)
    messageStore.message = 'Regenerating metadata...'
    messageStore.warn = false
    messageStore.showMessage = true
  } catch (error) {
    console.error('Error:', error)
  }
}
</script>
