<template>
  <v-toolbar class="position-absolute my-toolbar">
    <v-btn
      v-if="route.meta.isViewPage"
      icon="mdi mdi-arrow-left"
      :to="leaveViewPage(route)"
    ></v-btn>
    <v-spacer></v-spacer>
    <v-btn icon="mdi-information-outline" @click="infoStore.showInfo = !infoStore.showInfo"></v-btn>
    <v-btn
      v-if="metadata && metadata.database"
      :icon="metadata.database.tag.includes('_favorite') ? 'mdi-star' : 'mdi-star-outline'"
      @click="quickEditTags('favorite')"
    ></v-btn>
    <v-btn
      v-if="metadata && metadata.database"
      :icon="
        metadata.database.tag.includes('_archived')
          ? 'mdi-archive-arrow-up-outline'
          : 'mdi-archive-arrow-down-outline'
      "
      @click="quickEditTags('archived')"
    ></v-btn>
    <v-menu v-if="metadata && metadata.database">
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props" icon="mdi-dots-vertical"></v-btn>
      </template>
      <v-list>
        <v-list-item
          prepend-icon="mdi-open-in-new"
          value="view-original-file"
          :href="getSrc(metadata.database.hash, true, metadata.database.ext, Cookies.get('jwt')!, undefined )"
          target="_blank"
        >
          <v-list-item-title class="wrap">{{ 'View Original File' }}</v-list-item-title>
        </v-list-item>
        <v-list-item
          prepend-icon="mdi-download"
          value="download-original-file"
          :href="getSrc(metadata.database.hash,  true, metadata.database.ext, Cookies.get('jwt')!, undefined)"
          :download="`${metadata.database.hash}.${metadata.database.ext}`"
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
          v-if="!metadata.database.tag.includes('_trashed')"
          prepend-icon="mdi-trash-can-outline"
          value="delete-file"
          @click="quickEditTags('trashed')"
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
          @click="regeneratePreview()"
        >
          <v-list-item-title class="wrap">{{ 'Regenerate Preview' }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-toolbar>
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed } from 'vue'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { AbstractData } from '@/script/common/types'
import { getSrc } from '@/../config'
import { useInfoStore } from '@/store/infoStore'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { useModalStore } from '@/store/modalStore'
import { leaveViewPage } from '@/script/navigator'

import axios from 'axios'
import Cookies from 'js-cookie'

const modalStore = useModalStore('')
const infoStore = useInfoStore('')

const props = defineProps<Props>()
interface Props {
  isolationId: string
  hash: string
  index: number
  metadata: AbstractData | undefined
}

const route = useRoute()

const isViewPath = computed(() => {
  const path = route.path
  return (
    path.startsWith('/view') ||
    path.startsWith('/favorite/view') ||
    path.startsWith('/archived/view') ||
    path.startsWith('/all/view')
  )
})

function quickEditTags(category: 'favorite' | 'archived' | 'trashed') {
  let indexArray: number[] = []
  if (isViewPath.value) {
    indexArray = [props.index]
  }
  let removeTagsArray: string[] = []
  let addTagsArray: string[] = []
  if (category === 'favorite') {
    if (!props.metadata?.database!.tag.includes('_favorite')) {
      addTagsArray = ['_favorite']
    } else {
      removeTagsArray = ['_favorite']
    }
  } else if (category === 'archived') {
    if (!props.metadata?.database!.tag.includes('_archived')) {
      addTagsArray = ['_archived']
    } else {
      removeTagsArray = ['_archived']
    }
  } else if (category === 'trashed') {
    if (!props.metadata?.database!.tag.includes('_trashed')) {
      addTagsArray = ['_trashed']
    } else {
      removeTagsArray = ['_trashed']
    }
  }
  editTagsInWorker(indexArray, addTagsArray, removeTagsArray, props.isolationId)
}

const deleteData = () => {
  if (props.metadata) {
    if (props.index !== undefined) {
      deleteDataInWorker([props.index])
    }
  }
}

const regeneratePreview = async () => {
  if (props.metadata) {
    const hash = props.metadata.database!.hash
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
}
</script>
