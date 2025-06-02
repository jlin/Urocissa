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
import { storeHashToken } from '@/db/db'
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
  const token = tokenStore.hashTokenMap.get(props.database.hash)
  if (token !== undefined) {
    await storeHashToken(props.database.hash, token)
    tokenReady.value = true
  }
})
</script>
