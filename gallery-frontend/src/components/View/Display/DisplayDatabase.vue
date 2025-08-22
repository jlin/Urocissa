<template>
  <div
    v-if="abstractData && abstractData.database"
    id="col-ref"
    class="h-100 d-flex align-center justify-center"
  >
    <DisplayDatabaseImage
      :key="index"
      v-if="abstractData.database.ext_type === 'image'"
      :isolation-id="isolationId"
      :index="index"
      :abstract-data="abstractData"
    />

    <DisplayDatabaseVideo
      :key="index"
      v-if="abstractData.database.ext_type === 'video' && !abstractData.database.pending"
      :database="abstractData.database"
      :hash="abstractData.database.hash"
      :isolation-id="isolationId"
    />
    <v-card
      v-if="abstractData.database.ext_type === 'video' && abstractData.database.pending"
      class="d-flex align-center justify-start"
      outlined
      style="padding: 16px"
    >
      <div align="center" no-gutters>
        <div cols="auto" class="d-flex align-center">
          <v-icon size="48" color="warning">mdi-alert-circle-outline</v-icon>
        </div>
        <div class="text-left pl-4">
          <div>This video is currently being processed.</div>
          <div>Please check back later.</div>
        </div>
      </div>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { AbstractData, IsolationId } from '@type/types'

import { useCurrentFrameStore } from '@/store/currentFrameStore'
import { ref, watch } from 'vue'
import DisplayDatabaseVideo from './DisplayDatabaseVideo.vue'
import DisplayDatabaseImage from './DisplayDatabaseImage.vue'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  abstractData: AbstractData
}>()

const currentFrameStore = useCurrentFrameStore(props.isolationId)
const videoRef = ref<HTMLVideoElement | null>(null)

watch(videoRef, () => {
  currentFrameStore.video = videoRef.value
})
</script>
