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
import { Album, IsolationId } from '@/script/common/types'
import FindInTimeline from './Item/ItemFindInTimeline.vue'
import EditTags from './Item/ItemEditTags.vue'
import Delete from './Item/ItemDelete.vue'
import PermanentlyDelete from './Item/ItemPermanentlyDelete.vue'
import Restore from './Item/ItemRestore.vue'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  album: Album
}>()
</script>
