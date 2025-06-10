<template>
  <video
    v-if="tokenReady"
    controls
    autoplay
    :src="getSrc(hash, false, 'mp4', Cookies.get('jwt')!, undefined)"
    :style="{
      width: `${database.width}px`,
      height: `${database.height}px`,
      maxWidth: '100%',
      maxHeight: '100%'
    }"
    inline
    ref="videoRef"
    crossorigin="anonymous"
  >
    >
  </video>
</template>

<script setup lang="ts">
import { Database, IsolationId } from '@type/types'
import Cookies from 'js-cookie'
import { useCurrentFrameStore } from '@/store/currentFrameStore'
import { onMounted, ref, watch } from 'vue'
import { getSrc } from '@/../config'
import { useTokenStore } from '@/store/tokenStore'
const props = defineProps<{
  isolationId: IsolationId
  hash: string
  database: Database
}>()

const tokenReady = ref(false)

const tokenStore = useTokenStore(props.isolationId)
const currentFrameStore = useCurrentFrameStore(props.isolationId)

const videoRef = ref<HTMLVideoElement | null>(null)

watch(videoRef, () => {
  currentFrameStore.video = videoRef.value
})
onMounted(async () => {
  const success = await tokenStore.tryRefreshAndStoreTokenToDb(props.database.hash)
  if (!success) {
    throw new Error('Token renewal failed')
  }
  tokenReady.value = true
})
</script>
