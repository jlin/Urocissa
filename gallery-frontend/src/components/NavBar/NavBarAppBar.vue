<template>
  <v-app-bar v-if="!collectionStore.editModeOn">
    <v-btn v-if="!route.meta.isReadPage" @click="showDrawer = !showDrawer" icon="mdi-menu"> </v-btn>
    <v-btn
      v-else
      icon="mdi mdi-arrow-left"
      :to="albumStore.leaveAlbumPath ? albumStore.leaveAlbumPath : '/'"
    ></v-btn>
    <v-card
      v-if="
        route.meta.isReadPage && !route.meta.isViewPage && typeof route.params.hash === 'string'
      "
      elevation="0"
      class="w-50"
    >
      <v-card-title class="text-truncate">
        {{ albumStore.albumMap.get(route.params.hash) }}
      </v-card-title>
    </v-card>
    <v-card
      :style="{
        width: `${route.meta.isInsideAlbum ? '50%' : '100%'}`
      }"
    >
      <v-card-text class="pa-0">
        <v-text-field
          id="nav-search-input"
          rounded
          class="ma-0"
          v-model="searchQuery"
          bg-color="grey-darken-2"
          @click:prepend-inner="handleSearch"
          @click:clear="handleSearch"
          @keyup.enter="handleSearch"
          clearable
          persistent-clear
          variant="solo"
          flat
          prepend-inner-icon="mdi-magnify"
          single-line
          hide-details
          style="margin-right: 10px"
        >
          <template #label>
            <span class="text-caption">Search</span>
          </template>
        </v-text-field>
      </v-card-text>
    </v-card>

    <v-menu v-if="!route.meta.isInsideAlbum">
      <template #activator="{ props }">
        <v-btn v-bind="props" icon="mdi-plus"></v-btn>
      </template>
      <v-list>
        <v-list-item prepend-icon="mdi-upload" value="upload" @click="triggerFileInput">
          <v-list-item-title class="wrap">{{ 'Upload' }}</v-list-item-title>
        </v-list-item>
        <v-list-item prepend-icon="mdi-book-plus" value="create-album" @click="triggerModal()">
          <v-list-item-title class="wrap">{{ 'Create Album' }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-app-bar>
  <!-- If collectionStore.editModeOn === true then show the Editbar -->
  <EditBar v-else />
  <input
    v-if="!route.fullPath.includes('share')"
    id="upload-input"
    type="file"
    @change="handleFileUpload"
    ref="fileInput"
    multiple
    style="display: none"
  />
</template>

<script setup lang="ts">
import { inject, ref, type Ref, watchEffect } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useFilterStore } from '@/store/filterStore'
import { useMessageStore } from '@/store/messageStore'
import { useCollectionStore } from '@/store/collectionStore'
import EditBar from '@/components//NavBar/NavBarAppBarEditBar.vue'
import axios from 'axios'
import { useUploadStore } from '@/store/uploadStore'
import { useModalStore } from '@/store/modalStore'
import { useAlbumStore } from '@/store/albumStore'

const showDrawer = inject('showDrawer')

const albumStore = useAlbumStore('mainId')
const modalStore = useModalStore('mainId')
const uploadStore = useUploadStore('mainId')
const collectionStore = useCollectionStore('mainId')
const route = useRoute()
const router = useRouter()
const searchQuery = ref('')
const messageStor = useMessageStore('mainId')
const fileInput: Ref<HTMLInputElement | null> = ref(null)
const filterStore = useFilterStore('mainId')

const triggerModal = () => {
  modalStore.showCreateAlbumsModal = true
  console.log('modalStore.showCreateAlbumsModal is', modalStore.showCreateAlbumsModal)
}

watchEffect(() => {
  uploadStore.uploadButton = fileInput.value
})

const handleSearch = async () => {
  filterStore.filterString = searchQuery.value
  await router.replace({
    path: route.path,
    query: { search: searchQuery.value }
  })
}

watchEffect(() => {
  searchQuery.value = route.query.search as string
})

function triggerFileInput(): void {
  if (fileInput.value) {
    fileInput.value.click()
  }
}

async function handleFileUpload(event: Event): Promise<void> {
  const messageStore = useMessageStore('mainId')
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || files.length === 0) return

  let formData = new FormData()
  let totalSize = 0

  Array.from(files).forEach((file, i) => {
    formData.append('lastModified' + i.toString(), file.lastModified.toString())
    formData.append('file' + i.toString(), file)
    totalSize += file.size
  })

  console.log(`Total upload size: ${totalSize.toString()} bytes`)

  try {
    const startTime = Date.now()

    uploadStore.total = 0
    uploadStore.loaded = 0
    uploadStore.startTime = startTime
    uploadStore.uploading = true

    await axios.post('/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      },
      onUploadProgress: (progressEvent) => {
        if (progressEvent.total !== undefined) {
          uploadStore.total = progressEvent.total
          uploadStore.loaded = progressEvent.loaded
          uploadStore.startTime = startTime

          console.log(`Upload is ${uploadStore.percentComplete().toString()}% complete`)
          console.log(`Remaining time: ${uploadStore.remainingTime().toString()} seconds`)
        }
      }
    })

    messageStore.message = 'Files uploaded successfully!'
    messageStore.warn = false
    messageStore.showMessage = true
  } catch (error) {
    console.error('There was an error uploading the files: ', error)

    if (error instanceof Error) {
      messageStor.message = `There was an error uploading the files: ${error.message}`
    } else {
      messageStor.message = `There was an error uploading the files: ${String(error)}`
    }

    messageStor.warn = true
    messageStor.showMessage = true
  }
}
</script>
