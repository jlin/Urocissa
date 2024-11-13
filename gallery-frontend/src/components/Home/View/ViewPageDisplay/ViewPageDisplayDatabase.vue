<template>
  <v-col
    v-if="metadata && metadata.database"
    id="col-ref"
    class="h-100 d-flex align-center justify-center"
  >
    <img
      :key="index"
      v-if="metadata.database.ext_type === 'image' && imgStore.imgOriginal.get(index)"
      :src="imgStore.imgOriginal.get(index)"
      :style="{
        width: `${metadata.database.width}px`,
        height: `${metadata.database.height}px`,
        maxWidth: '100%',
        maxHeight: '100%',
        objectFit: 'scale-down'
      }"
    />
    <video
      controls
      autoplay
      v-if="metadata.database.ext_type === 'video' && !metadata.database.pending"
      :src="getSrc(hash, false, 'mp4', Cookies.get('jwt')!, undefined)"
      :style="{
        width: `${metadata.database.width}px`,
        height: `${metadata.database.height}px`,
        maxWidth: '100%',
        maxHeight: '100%'
      }"
      inline
    ></video>
  </v-col>
</template>

<script setup lang="ts">
import { VCol } from 'vuetify/components'
import { useImgStore } from '@/store/imgStore'
import Cookies from 'js-cookie'
import { getSrc } from '@/../config.ts'
import { AbstractData } from '@/script/common/types'

const props = defineProps<{
  isolationId: string
  hash: string
  index: number
  metadata: AbstractData
  colWidth: number
  colHeight: number
}>()

const imgStore = useImgStore(props.isolationId)
</script>
