<template>
  <v-toolbar class="position-absolute my-toolbar">
    <v-btn v-if="route.meta.isViewPage" icon="mdi mdi-arrow-left" :to="leavePage(route)"></v-btn>
    <v-spacer></v-spacer>
    <v-btn icon="mdi-information-outline" @click="infoStore.showInfo = !infoStore.showInfo"></v-btn>
    <v-btn
      v-if="metadata && metadata.database"
      :icon="metadata.database.tag.includes('_favorite') ? 'mdi-star' : 'mdi-star-outline'"
      @click="
        metadata.database.tag.includes('_favorite')
          ? quickRemoveTags('_favorite', index, isolationId)
          : quickAddTags('_favorite', index, isolationId)
      "
    ></v-btn>
    <v-btn
      v-if="metadata && metadata.database"
      :icon="
        metadata.database.tag.includes('_archived')
          ? 'mdi-archive-arrow-up-outline'
          : 'mdi-archive-arrow-down-outline'
      "
      @click="
        metadata.database.tag.includes('_archived')
          ? quickRemoveTags('_archived', index, isolationId)
          : quickAddTags('_archived', index, isolationId)
      "
    ></v-btn>
    <ViewPageToolBarDatabase
      v-if="metadata && metadata.database"
      :database="metadata.database"
      :index="index"
      :hash="hash"
      :isolation-id="isolationId"
    />
    <ViewPageToolBarAlbum
      v-if="metadata && metadata.album"
      :album="metadata.album"
      :index="index"
      :hash="hash"
      :isolation-id="isolationId"
    />
  </v-toolbar>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router'
import { quickRemoveTags, quickAddTags } from '@/script/common/quickEditTags'
import { AbstractData } from '@/script/common/types'
import { useInfoStore } from '@/store/infoStore'
import { leavePage } from '@/script/navigator'
import ViewPageToolBarDatabase from './ViewPageToolBarDatabase.vue'
import ViewPageToolBarAlbum from './ViewPageToolBarAlbum.vue'

const props = defineProps<{
  isolationId: string
  hash: string
  index: number
  metadata: AbstractData | undefined
}>()
const infoStore = useInfoStore(props.isolationId)

const route = useRoute()
</script>
