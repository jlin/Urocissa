<template>
  <v-app-bar v-if="!collectionStore.editModeOn">
    <v-btn @click="showDrawer = !showDrawer" icon="mdi-menu"> </v-btn>
    <v-card class="w-100">
      <v-card-text>
        <v-text-field
          id="nav-search-input"
          rounded
          class="ma-2"
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
    <v-btn v-if="!route.fullPath.includes('share')" @click="triggerFileInput" icon="mdi-upload">
    </v-btn>
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

const showDrawer = inject('showDrawer') as Ref<boolean>
const uploadStore = useUploadStore()
const collectionStore = useCollectionStore()
const route = useRoute()
const router = useRouter()
const searchQuery = ref('')
const messageStor = useMessageStore()
const fileInput: Ref<HTMLInputElement | null> = ref(null)
const filterStore = useFilterStore()

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

    const response = await axios.post('/upload', formData, {
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

    const result = response.data
    if (result.status === 'ok') {
      console.log('Upload complete')
      uploadStore.uploading = false
      messageStor.message = 'Files uploaded successfully!'
      messageStor.showMessage = true
    } else if (result.status === 'error') {
      messageStor.message = result.message
      messageStor.warn = true
      messageStor.showMessage = true
    }
  } catch (error) {
    console.error('There was an error uploading the files: ', error)
    messageStor.message = `There was an error uploading the files: ${error}`
    messageStor.warn = true
    messageStor.showMessage = true
  }
}
</script>
