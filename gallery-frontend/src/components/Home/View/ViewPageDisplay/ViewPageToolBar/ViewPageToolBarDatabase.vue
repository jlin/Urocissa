<template>
  <v-menu>
    <template #activator="{ props: MenuBtn }">
      <v-btn v-bind="MenuBtn" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <v-list-item
        prepend-icon="mdi-open-in-new"
        value="view-original-file"
        :href="getSrc(database.hash, true, database.ext, Cookies.get('jwt')!, undefined )"
        target="_blank"
      >
        <v-list-item-title class="wrap">{{ 'View Original File' }}</v-list-item-title>
      </v-list-item>
      <v-list-item
        prepend-icon="mdi-download"
        value="download-original-file"
        :href="getSrc(database.hash,  true, database.ext, Cookies.get('jwt')!, undefined)"
        :download="`${database.hash}.${database.ext}`"
      >
        <v-list-item-title class="wrap">{{ 'Download Original File' }}</v-list-item-title>
      </v-list-item>
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
        prepend-icon="mdi-image-album"
        value="edit-albums"
        @click="modalStore.showEditAlbumsModal = true"
      >
        <v-list-item-title class="wrap">{{ 'Edit Albums' }}</v-list-item-title>
      </v-list-item>
      <v-list-item
        v-if="!database.tag.includes('_trashed')"
        prepend-icon="mdi-trash-can-outline"
        value="delete-file"
        @click="quickAddTags('_trashed', index, isolationId)"
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
      <v-divider></v-divider>
      <v-list-item
        prepend-icon="mdi-image-refresh-outline"
        value="regenerate-preview"
        @click="regeneratePreview(database)"
      >
        <v-list-item-title class="wrap">{{ 'Regenerate Preview' }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>
<script setup lang="ts">
import { quickAddTags } from '@/script/common/quickEditTags'
import { DataBase } from '@/script/common/types'
import { getSrc } from '@/../config'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { useModalStore } from '@/store/modalStore'
import axios from 'axios'
import Cookies from 'js-cookie'

const props = defineProps<{
  isolationId: string
  hash: string
  index: number
  database: DataBase
}>()
const modalStore = useModalStore('mainId')

const deleteData = () => {
  deleteDataInWorker([props.index], props.isolationId)
}

const regeneratePreview = async (database: DataBase) => {
  const hash = database.hash
  const data = [hash] // Replace with your actual data

  try {
    const response = await axios.post('/put/regenerate-preview', data, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    console.log('Response:', response.data)
  } catch (error) {
    console.error('Error:', error)
  }
}
</script>
