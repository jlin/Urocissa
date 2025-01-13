<template>
  <div
    ref="dropZoneRef"
    id="dropzone"
    class="w-100 h-100 position-absolute d-flex justify-center align-center"
    :v-show="visible"
    :style="{
      backgroundColor: 'rgba(255, 255, 255, 0.5)',
      pointerEvents: visible ? 'auto' : 'none',
      opacity: visible ? 1 : 0,
      zIndex: 2000,
      transition: 'opacity 0.4s ease'
    }"
  >
    <v-card class="pa-16 d-flex flex-column align-center" outlined elevation="10">
      <v-icon size="128" icon="mdi-cloud-upload" class="mb-5" />
      <div class="mt-3 text-center" style="font-size: 2rem; font-weight: bold">
        Drag and drop files here
      </div>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { useUploadStore } from '@/store/uploadStore'
import { useDropZone } from '@vueuse/core'
import { onMounted, ref, watch } from 'vue'
const uploadStore = useUploadStore('mainId')
const visible = ref(false)
const dropZoneRef = ref<HTMLDivElement>()
function onDrop(files: File[] | null) {
  if (files !== null) {
    uploadStore
      .fileUpload(files)
      .then((result) => {
        console.log(result)
      })
      .catch((error: unknown) => {
        console.error('Error occurred:', error)
      })
  }
}

const { isOverDropZone } = useDropZone(dropZoneRef, {
  onDrop,
  // control multi-file drop
  multiple: true,
  // whether to prevent default behavior for unhandled events
  preventDefaultForUnhandled: false
})

watch(isOverDropZone, () => {
  if (!isOverDropZone.value) {
    visible.value = false
  }
})

onMounted(() => {
  window.addEventListener('dragenter', (event: DragEvent) => {
    if (event.dataTransfer) {
      const itemsArray: DataTransferItem[] = Array.from(event.dataTransfer.items)

      const hasValidType = itemsArray.some(
        (item) =>
          item.type.startsWith('image/') || item.type.startsWith('video/') || item.type === ''
      )
      visible.value = hasValidType
    }
  })
  window.addEventListener('dragend', () => {
    visible.value = false
  })
})
</script>
