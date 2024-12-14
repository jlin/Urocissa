<!-- NavBarAppBarEditBarMenuNormal.vue -->
<template>
  <v-menu>
    <template #activator="{ props }">
      <v-btn v-bind="props" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <SetAsCover v-if="route.meta.isReadPage && collectionStore.editModeCollection.size === 1" />
      <v-divider
        v-if="route.meta.isReadPage && collectionStore.editModeCollection.size === 1"
      ></v-divider>
      <Archive />
      <Favorite />
      <BatchEditTags />
      <BatchEditAlbums />
      <v-divider></v-divider>
      <Download />
      <v-divider></v-divider>
      <v-list-item
        v-if="!route.path.startsWith('/trashed')"
        prepend-icon="mdi-trash-can-outline"
        @click="handleQuickEdit('trashed')"
      >
        <v-list-item-title class="wrap">Delete</v-list-item-title>
      </v-list-item>

      <v-list-item v-else prepend-icon="mdi-trash-can-outline" @click="deleteData">
        <v-list-item-title class="wrap">Permanently Delete</v-list-item-title>
      </v-list-item>

      <v-divider></v-divider>

      <v-list-item prepend-icon="mdi-image-refresh-outline" @click="regeneratePreview">
        <v-list-item-title class="wrap">Regenerate Preview</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router'
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import axios from 'axios'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'

import { getIsolationIdByRoute } from '@/script/common/functions'

import SetAsCover from './Item/SetAsCover.vue'
import Archive from './Item/Archive.vue'
import Favorite from './Item/Favorite.vue'
import BatchEditTags from './Item/BatchEditTags.vue'
import BatchEditAlbums from './Item/BatchEditAlbums.vue'
import Download from './Item/Download.vue'
const route = useRoute()
const isolationId = getIsolationIdByRoute(route)
const collectionStore = useCollectionStore(isolationId)
const prefetchStore = usePrefetchStore(isolationId)
// Methods
const handleQuickEdit = (category: 'favorite' | 'archive' | 'trashed') => {
  const indexArray = Array.from(collectionStore.editModeCollection)

  let removeTagsArray: string[] = []
  let addTagsArray: string[] = []

  if (category === 'favorite') {
    addTagsArray = ['_favorite']
  } else if (category === 'archive') {
    addTagsArray = ['_archived']
  } else {
    addTagsArray = ['_trashed']
  }

  editTagsInWorker(indexArray, addTagsArray, removeTagsArray, isolationId)
}

const regeneratePreview = async () => {
  const indexArray = Array.from(collectionStore.editModeCollection)
  const regenerateData = {
    indexArray: indexArray,
    timestamp: prefetchStore.timestamp
  }
  try {
    const response = await axios.post('/put/regenerate-preview', regenerateData, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    console.log('Response:', response.data)
  } catch (error) {
    console.error('Error:', error)
  }
}

const deleteData = () => {
  const indexArray = Array.from(collectionStore.editModeCollection)
  deleteDataInWorker(indexArray, isolationId)
}
</script>
