<template>
  <v-app-bar v-if="!collectionStore.editModeOn" scroll-off-screen>
    <v-row justify="center" align="center" class="w-100 h-100" no-gutters>
      <v-col justify="center" align="center" cols="1">
        <v-btn v-if="!route.meta.isInsideAlbum" @click="showDrawer = !showDrawer" icon="mdi-menu">
        </v-btn>
        <v-btn v-else icon="mdi mdi-arrow-left" to="/albums"></v-btn>
      </v-col>
      <v-col v-if="route.meta.isInsideAlbum" cols="5">
        <v-card elevation="0">
          <v-card-title class="text-truncate">
            {{ albumStore.albumMap.get(route.path.split('/')[1].replace('album-', '')) }}
          </v-card-title>
        </v-card>
      </v-col>
      <v-col :cols="route.meta.isInsideAlbum ? 5 : 10" justify="center" align="center">
        <v-card class="w-100">
          <v-card-text class="pa-0">
            <v-text-field
              id="nav-search-input"
              rounded
              class="ma-0"
              v-model="searchQuery"
              bg-color="grey-darken-2"
              @click:prependInner="handleSearch"
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
              <template v-slot:label>
                <span class="text-caption">Search</span>
              </template>
            </v-text-field>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="1" justify="center" align="center">
        <v-menu>
          <template v-slot:activator="{ props }">
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
      </v-col>
    </v-row>
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

const albumStore = useAlbumStore()
const modalStore = useModalStore()
const showDrawer = inject('showDrawer') as Ref<boolean>
const uploadStore = useUploadStore()
const collectionStore = useCollectionStore()
const route = useRoute()
const router = useRouter()
const searchQuery = ref('')
const messageStor = useMessageStore()
const fileInput: Ref<HTMLInputElement | null> = ref(null)
const filterStore = useFilterStore()

const triggerModal = () => {
  modalStore.showCreateAlbumsModal = true
  console.log('modalStore.showCreateAlbumsModal is', modalStore.showCreateAlbumsModal)
}

watchEffect(() => {
  console.log('modalStore.showCreateAlbumsModal is', modalStore.showCreateAlbumsModal)
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
  const messageStore = useMessageStore()
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || files.length === 0) return

  let formData = new FormData()
  let totalSize = 0

  for (let i = 0; i < files.length; i++) {
    formData.append('lastModifiede' + i, files[i].lastModified.toString())
    formData.append('file' + i, files[i])
    totalSize += files[i].size
  }

  console.log(`Total upload size: ${totalSize} bytes`)

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
        if (progressEvent.total) {
          uploadStore.total = progressEvent.total
          uploadStore.loaded = progressEvent.loaded
          uploadStore.startTime = startTime

          console.log(`Upload is ${uploadStore.percentComplete()}% complete`)
          console.log(`Remaining time: ${uploadStore.remainingTime()} seconds`)
        } else {
          console.log(`Uploaded ${uploadStore.loaded} bytes`)
        }
      }
    })

    messageStore.message = 'Files uploaded successfully!'
    messageStore.warn = false
    messageStore.showMessage = true
  } catch (error) {
    console.error('There was an error uploading the files: ', error)
    messageStor.message = `There was an error uploading the files: ${error}`
    messageStor.warn = true
    messageStor.showMessage = true
  }
}
</script>
