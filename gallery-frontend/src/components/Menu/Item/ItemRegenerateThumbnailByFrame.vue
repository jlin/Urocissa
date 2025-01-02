<template>
  <v-list-item prepend-icon="mdi-image-refresh-outline" @click="regenerateThumbnailByFrame">
    <v-list-item-title class="wrap">Capture Frame as Thumb</v-list-item-title>
  </v-list-item>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import axios from 'axios'
import { getCookiesJwt, getIsolationIdByRoute } from '@/script/common/functions'
import { useCurrentFrameStore } from '@/store/currentFrameStore'
import { getSrc } from '@/../config'

const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const currentFrameStore = useCurrentFrameStore(isolationId)

const regenerateThumbnailByFrame = async () => {
  const hash = route.params.hash
  const blob = await currentFrameStore.getCapture()
  if (typeof hash === 'string' && blob) {
    const formData = new FormData()

    // Append the hash first
    formData.append('hash', hash)

    // Append the file
    formData.append('file', blob)

    const response = await axios.put('/put/regenerate-thumbnail-with-frame', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })
    await axios.get<Blob>(getSrc(hash, false, 'jpg', getCookiesJwt(), undefined), {
      responseType: 'blob',
      headers: {
        'Cache-Control': 'no-cache, no-store, must-revalidate',
        Pragma: 'no-cache',
        Expires: '0'
      }
    })
    console.log('Response:', response.data)
  }
}
</script>
