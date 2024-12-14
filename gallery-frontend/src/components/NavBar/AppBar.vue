<template>
  <v-toolbar
    flat
    height="2"
    class="no-select"
    :style="{
      backgroundColor: '#212121'
    }"
  >
  </v-toolbar>
  <InfoBar v-if="!collectionStore.editModeOn" />
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
import { ref, type Ref, watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { useMessageStore } from '@/store/messageStore'
import { useCollectionStore } from '@/store/collectionStore'
import EditBar from '@/components/NavBar/EditBar.vue'
import InfoBar from '@/components/NavBar/InfoBar.vue'
import axios from 'axios'
import { useUploadStore } from '@/store/uploadStore'

const uploadStore = useUploadStore('mainId')
const collectionStore = useCollectionStore('mainId')
const route = useRoute()

const searchQuery = ref('')
const messageStor = useMessageStore('mainId')
const fileInput: Ref<HTMLInputElement | null> = ref(null)

watchEffect(() => {
  uploadStore.uploadButton = fileInput.value
})

watchEffect(() => {
  searchQuery.value = route.query.search as string
})

async function handleFileUpload(event: Event): Promise<void> {
  const messageStore = useMessageStore('mainId')
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || files.length === 0) return

  let formData = new FormData()
  let totalSize = 0

  Array.from(files).forEach((file, i) => {
    formData.append(`lastModified${i}`, `${file.lastModified}`)
    formData.append(`file${i}`, file)
    totalSize += file.size
  })

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
        if (progressEvent.total !== undefined) {
          uploadStore.total = progressEvent.total
          uploadStore.loaded = progressEvent.loaded
          uploadStore.startTime = startTime

          console.log(`Upload is ${uploadStore.percentComplete()}% complete`)
          console.log(`Remaining time: ${uploadStore.remainingTime()} seconds`)
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
