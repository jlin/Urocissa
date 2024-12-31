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
import { useCurrentFrameStore } from '@/store/currentFrameStore'

const props = defineProps<{
  index: number
  hash: string
}>()

const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const prefetchStore = usePrefetchStore(isolationId)
const messageStore = useMessageStore('mainId')
const currentFrameStore = useCurrentFrameStore(isolationId)

const setPreviewByCurrentFrame = async () => {
  currentFrameStore.getCapture()
}
</script>
