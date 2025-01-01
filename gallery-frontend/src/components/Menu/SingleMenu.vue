<template>
  <v-menu location="start">
    <template #activator="{ props: MenuBtn }">
      <v-btn v-bind="MenuBtn" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <ItemViewOriginalFile
        :src="getSrc(database.hash, true, database.ext, Cookies.get('jwt')!, undefined )"
      />
      <ItemDownload :index-list="[props.index]" />
      <ItemFindInTimeline :hash="props.hash" />
      <v-divider></v-divider>
      <ItemEditTags />
      <ItemEditAlbums />
      <ItemDelete v-if="!database.tag.includes('_trashed')" :index-list="[props.index]" />
      <ItemRestore v-if="database.tag.includes('_trashed')" :index-list="[props.index]" />
      <ItemPermanentlyDelete v-if="database.tag.includes('_trashed')" :index-list="[props.index]" />
      <v-divider></v-divider>
      <ItemRegeneratePreview :index-list="[props.index]" />
      <ItemRegenerateMetadata :index-list="[props.index]" />
      <ItemRegeneratePreviewByFrame v-if="currentFrameStore.video !== null" />
    </v-list>
  </v-menu>
</template>
<script setup lang="ts">
import { DataBase, IsolationId } from '@/script/common/types'
import { getSrc } from '@/../config'
import Cookies from 'js-cookie'
import ItemViewOriginalFile from './Item/ItemViewOriginalFile.vue'
import ItemDownload from './Item/ItemDownload.vue'
import ItemFindInTimeline from './Item/ItemFindInTimeline.vue'
import ItemEditTags from './Item/ItemEditTags.vue'
import ItemEditAlbums from './Item/ItemEditAlbums.vue'
import ItemDelete from './Item/ItemDelete.vue'
import ItemPermanentlyDelete from './Item/ItemPermanentlyDelete.vue'
import ItemRegeneratePreview from './Item/ItemRegeneratePreview.vue'
import ItemRegenerateMetadata from './Item/ItemRegenerateMetadata.vue'
import ItemRestore from './Item/ItemRestore.vue'
import ItemRegeneratePreviewByFrame from './Item/ItemRegeneratePreviewByFrame.vue'
import { useCurrentFrameStore } from '@/store/currentFrameStore'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  database: DataBase
}>()

const currentFrameStore = useCurrentFrameStore(props.isolationId)
</script>
