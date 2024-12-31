<template>
  <v-menu>
    <template #activator="{ props: MenuBtn }">
      <v-btn v-bind="MenuBtn" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <ViewOriginalFile
        :src="getSrc(database.hash, true, database.ext, Cookies.get('jwt')!, undefined )"
      />
      <Download :index-list="[props.index]" />
      <FindInTimeline :hash="props.hash" />
      <v-divider></v-divider>
      <EditTags />
      <EditAlbums />
      <Delete v-if="!database.tag.includes('_trashed')" :index-list="[props.index]" />
      <Restore v-if="database.tag.includes('_trashed')" :index-list="[props.index]" />
      <PermanentlyDelete
        v-if="database.tag.includes('_trashed')"
        :index-list="[props.index]"
      ></PermanentlyDelete>
      <v-divider></v-divider>
      <RegeneratePreview :index-list="[props.index]" />
      <RegenerateMetadata :index-list="[props.index]" />
      <ItemSetPreviewByCurrentFrane
        v-if="currentFrameStore.currentFrame !== undefined"
        :index="props.index"
        :current-frame="currentFrameStore.currentFrame"
      />
    </v-list>
  </v-menu>
</template>
<script setup lang="ts">
import { DataBase, IsolationId } from '@/script/common/types'
import { getSrc } from '@/../config'
import Cookies from 'js-cookie'
import ViewOriginalFile from './Item/ItemViewOriginalFile.vue'
import Download from './Item/ItemDownload.vue'
import FindInTimeline from './Item/ItemFindInTimeline.vue'
import EditTags from './Item/ItemEditTags.vue'
import EditAlbums from './Item/ItemEditAlbums.vue'
import Delete from './Item/ItemDelete.vue'
import PermanentlyDelete from './Item/ItemPermanentlyDelete.vue'
import RegeneratePreview from './Item/ItemRegeneratePreview.vue'
import RegenerateMetadata from './Item/ItemRegenerateMetadata.vue'
import Restore from './Item/ItemRestore.vue'
import ItemSetPreviewByCurrentFrane from './Item/ItemSetPreviewByCurrentFrane.vue'
import { useCurrentFrameStore } from '@/store/currentFrameStore'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  database: DataBase
}>()

const currentFrameStore = useCurrentFrameStore(props.isolationId)
</script>
