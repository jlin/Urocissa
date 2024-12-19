<template>
  <v-menu>
    <template #activator="{ props: MenuBtn }">
      <v-btn v-bind="MenuBtn" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <FindInTimeline :hash="props.hash" />
      <v-divider></v-divider>
      <EditTags />
      <Delete v-if="!album.tag.includes('_trashed')" :index-list="[props.index]" />
      <Restore v-if="album.tag.includes('_trashed')" :index-list="[props.index]" />
      <PermanentlyDelete v-if="album.tag.includes('_trashed')" :index-list="[props.index]" />
    </v-list>
  </v-menu>
</template>

<script setup lang="ts">
import { Album } from '@/script/common/types'
import FindInTimeline from './Item/FindInTimeline.vue'
import EditTags from './Item/EditTags.vue'
import Delete from './Item/Delete.vue'
import PermanentlyDelete from './Item/PermanentlyDelete.vue'
import Restore from './Item/Restore.vue'

const props = defineProps<{
  isolationId: string
  hash: string
  index: number
  album: Album
}>()
</script>
