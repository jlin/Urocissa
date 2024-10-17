<template>
  <v-toolbar class="position-absolute my-toolbar">
    <v-btn icon="mdi mdi-arrow-left" :to="{ path: computedPath, query: route.query }"></v-btn>
    <v-spacer></v-spacer>
    <v-btn icon="mdi-information-outline" @click="infoStore.showInfo = !infoStore.showInfo"></v-btn>
    <v-btn
      v-if="metadata"
      :icon="metadata.tag.includes('_favorite') ? 'mdi-star' : 'mdi-star-outline'"
      @click="quickEditTags('favorite')"
    ></v-btn>
    <v-btn
      v-if="metadata"
      :icon="
        metadata.tag.includes('_archived')
          ? 'mdi-archive-arrow-up-outline'
          : 'mdi-archive-arrow-down-outline'
      "
      @click="quickEditTags('archived')"
    ></v-btn>
    <v-menu v-if="metadata">
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props" icon="mdi-dots-vertical"></v-btn>
      </template>
      <v-list>
        <v-list-item
          prepend-icon="mdi-open-in-new"
          value="view-original-file"
          :href="getSrc(props.metadata!.hash, true, props.metadata!.ext, Cookies.get('password')!, undefined )"
          target="_blank"
        >
          <v-list-item-title class="wrap">{{ 'View Original File' }}</v-list-item-title>
        </v-list-item>
        <v-list-item
          prepend-icon="mdi-download"
          value="download-original-file"
          :href="getSrc(props.metadata!.hash,  true, props.metadata!.ext, Cookies.get('password')!, undefined)"
          :download="`${props.metadata!.hash}.${props.metadata!.ext}`"
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
          v-if="!metadata.tag.includes('_trashed')"
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
import { useDataStore } from '@/store/dataStore'
import { useCollectionStore } from '@/store/collectionStore'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { type DataBase } from '@/script/common/commonType'
import { getSrc } from '@/../config'
import { useInfoStore } from '@/store/infoStore'
import { deleteDataInWorker } from '@/script/inWorker/deleteDataInWorker'
import { useModalStore } from '@/store/modalStore'
import axios from 'axios'
import Cookies from 'js-cookie'
const modalStore = useModalStore()
const infoStore = useInfoStore()

const props = defineProps<Props>()
interface Props {
  metadata: DataBase | undefined
}
const hash = computed(() => {
  return route.params.hash as string
})
const dataStore = useDataStore()
const collectionStore = useCollectionStore()
const route = useRoute()
const computedPath = computed(() => {
  const path = route.path
  if (path.startsWith('/view')) {
    return '/'
  } else if (path.startsWith('/favorite/view')) {
    return '/favorite'
  } else if (path.startsWith('/archived/view')) {
    return '/archived'
  } else if (path.startsWith('/trashed/view')) {
    return '/trashed'
  } else if (path.startsWith('/all/view')) {
    return '/all'
  } else {
    return '/'
  }
})
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
    const index = dataStore.hashMapData.get(route.params.hash as string)!
    indexArray = [index]
  } else {
    indexArray = Array.from(collectionStore.editModeCollection)
  }
  let removeTagsArray: string[] = []
  let addTagsArray: string[] = []
  if (category === 'favorite') {
    if (!props.metadata?.tag.includes('_favorite')) {
      addTagsArray = ['_favorite']
    } else {
      removeTagsArray = ['_favorite']
    }
  } else if (category === 'archived') {
    if (!props.metadata?.tag.includes('_archived')) {
      addTagsArray = ['_archived']
    } else {
      removeTagsArray = ['_archived']
    }
  } else if (category === 'trashed') {
    if (!props.metadata?.tag.includes('_trashed')) {
      addTagsArray = ['_trashed']
    } else {
      removeTagsArray = ['_trashed']
    }
  }
  editTagsInWorker(indexArray, addTagsArray, removeTagsArray)
}

const deleteData = () => {
  if (props.metadata) {
    const index = dataStore.hashMapData.get(props.metadata.hash)
    if (index !== undefined) {
      deleteDataInWorker([index])
    }
  }
}

const regeneratePreview = async () => {
  if (props.metadata) {
    const hash = props.metadata.hash
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
