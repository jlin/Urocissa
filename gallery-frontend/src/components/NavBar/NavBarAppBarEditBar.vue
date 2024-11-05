<template>
  <v-app-bar>
    <v-btn icon="mdi-close" @click="leaveEdit"></v-btn>
    <v-card
      variant="flat"
      class="w-100"
      :title="`${collectionStore.editModeCollection.size} items`"
    >
    </v-card>
    <v-spacer></v-spacer>
    <v-btn
      v-if="prefetchStore.dataLength !== collectionStore.editModeCollection.size"
      icon="mdi-select-all"
      @click="selectAll()"
    ></v-btn>
    <v-btn v-else icon="mdi-select-remove" @click="selectRemove()"></v-btn>
    <v-btn icon="mdi-select-inverse" @click="selectInverse()"></v-btn>
    <v-menu>
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props" icon="mdi-dots-vertical"></v-btn>
      </template>
      <v-list>
        <v-list-item
          prepend-icon="mdi-archive-arrow-down"
          value="archive"
          @click="quickEditTags('archive')"
        >
          <v-list-item-title class="wrap">{{ 'Archive' }}</v-list-item-title>
        </v-list-item>
        <v-list-item
          prepend-icon="mdi-star-outline"
          value="favorite"
          @click="quickEditTags('favorite')"
        >
          <v-list-item-title class="wrap">{{ 'Favorite' }}</v-list-item-title>
        </v-list-item>
        <v-list-item
          prepend-icon="mdi-pencil"
          value="edit"
          @click="modalStore.showBatchEditTagsModal = true"
        >
          <v-list-item-title class="wrap">{{ 'Edit' }}</v-list-item-title>
        </v-list-item>
        <v-divider></v-divider>
        <v-list-item prepend-icon="mdi-download" value="download" @click="downloadAllFiles()">
          <v-list-item-title class="wrap">{{ 'Download' }}</v-list-item-title>
        </v-list-item>
        <v-divider></v-divider>
        <v-list-item
          v-if="!route.path.startsWith('/trashed')"
          prepend-icon="mdi-trash-can-outline"
          value="delete"
          @click="quickEditTags('trashed')"
        >
          <v-list-item-title class="wrap">{{ 'Delete' }}</v-list-item-title>
        </v-list-item>
        <v-list-item
          v-else
          prepend-icon="mdi-trash-can-outline"
          value="permanently-delete"
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
  </v-app-bar>
</template>
<script lang="ts" setup>
import { getSrc } from '@/../config'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { useCollectionStore } from '@/store/collectionStore'
import { usePrefetchStore } from '@/store/prefetchStore'
import { useDataStore } from '@/store/dataStore'
import { useModalStore } from '@/store/modalStore'
import axios from 'axios'
import { saveAs } from 'file-saver'
import Cookies from 'js-cookie'
import { useRoute } from 'vue-router'
const route = useRoute()
const modalStore = useModalStore()
const collectionStore = useCollectionStore()
const prefetchStore = usePrefetchStore()
const dataStore = useDataStore()
const leaveEdit = () => {
  collectionStore.editModeCollection.clear()
  collectionStore.editModeOn = false
}

const selectAll = () => {
  for (let i = 0; i < prefetchStore.dataLength; i++) {
    collectionStore.editModeCollection.add(i)
  }
}

const selectRemove = () => {
  collectionStore.editModeCollection.clear()
}

const selectInverse = () => {
  for (let i = 0; i < prefetchStore.dataLength; i++) {
    if (collectionStore.editModeCollection.has(i)) {
      collectionStore.editModeCollection.delete(i)
    } else {
      collectionStore.editModeCollection.add(i)
    }
  }
}

function quickEditTags(category: 'favorite' | 'archive' | 'trashed') {
  let indexArray = Array.from(collectionStore.editModeCollection)

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
        const url = getSrc(metadata.hash, true, metadata.ext, Cookies.get('jwt')!, undefined)
        const response = await axios.get(url, { responseType: 'blob' })
        const fileName = `${metadata.hash}.${metadata.ext}`
        saveAs(response.data, fileName)
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
