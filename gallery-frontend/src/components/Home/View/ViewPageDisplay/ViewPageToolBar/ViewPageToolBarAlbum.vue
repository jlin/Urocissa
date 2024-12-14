<template>
  <v-menu>
    <template #activator="{ props: MenuBtn }">
      <v-btn v-bind="MenuBtn" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <v-list-item prepend-icon="mdi-calendar-search-outline" :to="`/all?search=&locate=${hash}`">
        <v-list-item-title class="wrap">{{ 'Find In Timeline' }}</v-list-item-title>
      </v-list-item>
      <v-divider></v-divider>
      <v-list-item
        prepend-icon="mdi-pencil-outline"
        value="edit-tags"
        @click="modalStore.showEditTagsModal = true"
      >
        <v-list-item-title class="wrap">{{ 'Edit Tags' }}</v-list-item-title>
      </v-list-item>
      <v-list-item
        v-if="!album.tag.includes('_trashed')"
        prepend-icon="mdi-trash-can-outline"
        value="delete-file"
        @click="quickAddTags('_trashed', [index], isolationId)"
      >
        <v-list-item-title class="wrap">{{ 'Delete' }}</v-list-item-title>
      </v-list-item>
      <v-list-item
        v-else
        prepend-icon="mdi-trash-can-outline"
        value="permanently-delete-file"
        @click="deleteData()"
      >
        <v-list-item-title class="wrap">{{ 'Permanently Delete' }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>
<script setup lang="ts">
import { quickAddTags } from '@/script/common/quickEditTags'
import { Album } from '@/script/common/types'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { useModalStore } from '@/store/modalStore'

const props = defineProps<{
  isolationId: string
  hash: string
  index: number
  album: Album
}>()
const modalStore = useModalStore('mainId')

const deleteData = () => {
  deleteDataInWorker([props.index], props.isolationId)
}
</script>
