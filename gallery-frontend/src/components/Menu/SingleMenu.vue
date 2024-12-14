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
      <PermanentlyDelete v-else :index-list="[props.index]"></PermanentlyDelete>
      <v-divider></v-divider>
      <RegeneratePreview :index-list="[props.index]" />
    </v-list>
  </v-menu>
</template>
<script setup lang="ts">
import { DataBase } from '@/script/common/types'
import { getSrc } from '@/../config'
import Cookies from 'js-cookie'
import ViewOriginalFile from './Item/ViewOriginalFile.vue'
import Download from './Item/Download.vue'
import FindInTimeline from './Item/FindInTimeline.vue'
import EditTags from './Item/EditTags.vue'
import EditAlbums from './Item/EditAlbums.vue'
import Delete from './Item/Delete.vue'
import PermanentlyDelete from './Item/PermanentlyDelete.vue'
import RegeneratePreview from './Item/RegeneratePreview.vue'

const props = defineProps<{
  isolationId: string
  hash: string
  index: number
  database: DataBase
}>()
</script>
