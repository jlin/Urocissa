<!-- NavBarAppBarEditBarMenuNormal.vue -->
<template>
  <v-menu>
    <template #activator="{ props }">
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

      <v-list-item prepend-icon="mdi-image-album" @click="showBatchEditAlbumsModal">
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
import { batchNumber, getSrc } from '@/../config'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { fetchDataInWorker } from '@/script/inWorker/fetchDataInWorker'
import { getCookiesJwt, getIsolationIdByRoute } from '@/script/common/functions'
import { AbstractData } from '@/script/common/types'

const route = useRoute()
const collectionStore = useCollectionStore('mainId')
const prefetchStore = usePrefetchStore('mainId')
const dataStore = useDataStore('mainId')
const modalStore = useModalStore('mainId')

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

  editTagsInWorker(indexArray, addTagsArray, removeTagsArray, 'mainId')
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

const waitForMetadata = (index: number, timeout = 5000, interval = 100): Promise<AbstractData> => {
  console.log(`data with index ${index} not fetch; waiting...`)

  return new Promise((resolve, reject) => {
    const startTime = Date.now()

    const checkMetadata = () => {
      const metadata = dataStore.data.get(index)

      if (metadata) {
        console.log(`index ${index} waiting done`)
        resolve(metadata)
      } else if (Date.now() - startTime > timeout) {
        console.error(`index ${index} waiting timeout`)
        reject(new Error(`Timeout waiting for metadata at index ${index}`))
      } else {
        setTimeout(checkMetadata, interval)
      }
    }
    checkMetadata()
  })
}

const downloadAllFiles = async () => {
  const indexArray = Array.from(collectionStore.editModeCollection)
  const concurrencyLimit = 8
  const delay = 1000
  const delayFunction = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))
  const isolationId = getIsolationIdByRoute(route)
  try {
    for (let i = 0; i < indexArray.length; i += concurrencyLimit) {
      const batchIndex = indexArray.slice(i, i + concurrencyLimit)
      const downloadPromises = batchIndex.map(async (index) => {
        let metadata = dataStore.data.get(index)
        if (!metadata) {
          // Initiate data fetch
          fetchDataInWorker(Math.floor(index / batchNumber), isolationId)

          // Wait for metadata to be available
          try {
            metadata = await waitForMetadata(index)
          } catch (error) {
            console.error(error)
            return // Skip this index if metadata isn't available
          }
        }

        if (metadata.database) {
          const url = getSrc(
            metadata.database.hash,
            true,
            metadata.database.ext,
            getCookiesJwt(),
            undefined
          )
          try {
            const response = await axios.get<Blob>(url, { responseType: 'blob' })
            const fileName = `${metadata.database.hash}.${metadata.database.ext}`

            saveAs(response.data, fileName)
          } catch (downloadError) {
            console.error(`Failed to download file for index ${index}:`, downloadError)
          }
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
