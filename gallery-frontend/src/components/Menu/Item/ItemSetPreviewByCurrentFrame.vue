<template>
  <v-list-item prepend-icon="mdi-image-refresh-outline" @click="setPreviewByCurrentFrame">
    <v-list-item-title class="wrap">Set Preview By Current Frame</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { usePrefetchStore } from '@/store/prefetchStore'
import axios from 'axios'
import { getCookiesJwt, getIsolationIdByRoute } from '@/script/common/functions'
import { useMessageStore } from '@/store/messageStore'
import { getSrc } from '@/../config'

const props = defineProps<{
  index: number
  hash: string
  currentFrame: number
}>()

const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const prefetchStore = usePrefetchStore(isolationId)
const messageStore = useMessageStore('mainId')
const setPreviewByCurrentFrame = async () => {
  const regenerateWithFrame = {
    index: props.index,
    timestamp: prefetchStore.timestamp,
    frameSecond: props.currentFrame
  }
  try {
    const response = await axios.post('/put/regenerate-preview-with-frame', regenerateWithFrame, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    console.log('Response:', response.data)
    await axios.get<Blob>(getSrc(props.hash, false, 'jpg', getCookiesJwt(), undefined), {
      responseType: 'blob',
      headers: {
        'Cache-Control': 'no-cache, no-store, must-revalidate',
        Pragma: 'no-cache',
        Expires: '0'
      }
    })
    messageStore.message = 'Regenerating preview with frame...'
    messageStore.warn = false
    messageStore.showMessage = true
  } catch (error) {
    console.error('Error:', error)
  }
}
</script>
