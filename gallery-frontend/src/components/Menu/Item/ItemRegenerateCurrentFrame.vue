<template>
  <v-list-item prepend-icon="mdi-image-refresh-outline" @click="setPreviewByCurrentFrame">
    <v-list-item-title class="wrap">Set Preview By Current Frame</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import axios from 'axios'
import { getIsolationIdByRoute } from '@/script/common/functions'
import { useCurrentFrameStore } from '@/store/currentFrameStore'

const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const currentFrameStore = useCurrentFrameStore(isolationId)

const setPreviewByCurrentFrame = async () => {
  const hash = route.params.hash
  const blob = await currentFrameStore.getCapture()
  if (typeof hash === 'string' && blob) {
    const formData = new FormData()

    // Append the hash first
    formData.append('hash', hash)

    // Append the file
    formData.append('file', blob)

    // Send the request
    const response = await axios.put('/put/regenerate-preview-with-frame', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })

    console.log('Response:', response.data)
  }
}
</script>
