<!-- NavBarAppBarEditBarMenuNormal.vue -->
<template>
  <v-menu>
    <template v-slot:activator="{ props }">
      <v-btn v-bind="props" icon="mdi-dots-vertical"></v-btn>
    </template>
    <v-list>
      <v-list-item prepend-icon="mdi-archive-arrow-down" @click="handleQuickEdit('archive')">
        <v-list-item-title class="wrap">Archive</v-list-item-title>
      </v-list-item>

      <v-list-item prepend-icon="mdi-star-outline" @click="handleQuickEdit('favorite')">
        <v-list-item-title class="wrap">Favorite</v-list-item-title>
      </v-list-item>

      <v-list-item prepend-icon="mdi-tag" @click="showBatchEditTagsModal">
        <v-list-item-title class="wrap">Batch Edit Tags</v-list-item-title>
      </v-list-item>

      <v-list-item v-if="false"  prepend-icon="mdi-image-album" @click="showBatchEditAlbumsModal">
        <v-list-item-title class="wrap">Batch Edit Albums</v-list-item-title>
      </v-list-item>

      <v-divider></v-divider>

      <v-list-item prepend-icon="mdi-download" @click="downloadAllFiles">
        <v-list-item-title class="wrap">Download</v-list-item-title>
      </v-list-item>

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
import { useDataStore } from '@/store/dataStore'
import { useModalStore } from '@/store/modalStore'
import axios from 'axios'
import { saveAs } from 'file-saver'
import Cookies from 'js-cookie'
import { getSrc } from '@/../config'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'

const route = useRoute()
const collectionStore = useCollectionStore()
const prefetchStore = usePrefetchStore()
const dataStore = useDataStore()
const modalStore = useModalStore()

// Methods
const handleQuickEdit = (category: 'favorite' | 'archive' | 'trashed') => {
  const indexArray = Array.from(collectionStore.editModeCollection)

  let removeTagsArray: string[] = []
  let addTagsArray: string[] = []

  if (category === 'favorite') {
    addTagsArray = ['_favorite']
  } else if (category === 'archive') {
    addTagsArray = ['_archived']
  } else if (category === 'trashed') {
    addTagsArray = ['_trashed']
  }

  editTagsInWorker(indexArray, addTagsArray, removeTagsArray)
}

const showBatchEditTagsModal = () => {
  modalStore.showBatchEditTagsModal = true
}

const showBatchEditAlbumsModal = () => {
  modalStore.showBatchEditAlbumsModal = true
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
  deleteDataInWorker(indexArray)
}

const downloadAllFiles = async () => {
  const indexArray = Array.from(collectionStore.editModeCollection)
  const concurrencyLimit = 8
  const delay = 1000
  const delayFunction = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

  try {
    for (let i = 0; i < indexArray.length; i += concurrencyLimit) {
      const batchIndex = indexArray.slice(i, i + concurrencyLimit)
      const downloadPromises = batchIndex.map(async (index) => {
        const metadata = dataStore.data.get(index)!
        if (metadata.database) {
          const url = getSrc(
            metadata.database.hash,
            true,
            metadata.database.ext,
            Cookies.get('jwt')!,
            undefined
          )
          const response = await axios.get(url, { responseType: 'blob' })
          const fileName = `${metadata.database.hash}.${metadata.database.ext}`
          saveAs(response.data, fileName)
        }
      })

      await Promise.all(downloadPromises)
      await delayFunction(delay)
    }
    console.log('All files downloaded successfully')
  } catch (error) {
    console.error('Error downloading files:', error)
  }
}
</script>

<style scoped>
/* Add any component-specific styles here */
</style>
