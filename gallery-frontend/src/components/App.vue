<template>
  <v-app
    :style="{
      userSelect:
        scrollbarStore.isDragging || scrollbarStoreInsideAlbum.isDragging ? 'none' : 'auto' // Prevent accidental selection while scrolling.
    }"
  >
    <v-main class="h-screen">
      <div
        ref="dropZoneRef"
        id="dropzone"
        class="w-100 h-100 position-absolute d-flex justify-center align-center"
        :v-show="visible"
        :style="{
          backgroundColor: 'rgba(255, 255, 255, 0.5)',
          pointerEvents: visible ? 'auto' : 'none',
          opacity: visible ? 1 : 0,
          zIndex: 1000,
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
      <router-view v-slot="{ Component }" :key="routeKey">
        <component :is="Component" />
      </router-view>
    </v-main>
    <NotificationWarn />
  </v-app>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed, onMounted, ref, watch, watchEffect } from 'vue'
import { useScrollbarStore } from '@/store/scrollbarStore'
import NotificationWarn from '@/components/NotificationWarn.vue'
import { useRerenderStore } from '@/store/rerenderStore'

const scrollbarStore = useScrollbarStore('mainId')
const scrollbarStoreInsideAlbum = useScrollbarStore('subId')
const rerenderStore = useRerenderStore('mainId')
const route = useRoute()
const uploadStore = useUploadStore('mainId')
const visible = ref(false)
const currentPage = computed(() => {
  if (route.path.startsWith('/favorite')) {
    return 'favorite'
  } else if (route.path.startsWith('/archived')) {
    return 'archived'
  } else if (route.path.startsWith('/all')) {
    return 'all'
  } else {
    return 'default'
  }
})

// The routeKey is used to ensure that the router-view reloads the Home.vue component properly.
// Without it, Vue may cache the component for optimization, potentially causing bugs.
const routeKey = computed(() => {
  const search = typeof route.query.search === 'string' ? route.query.search : ''
  const locate = typeof route.query.locate === 'string' ? route.query.locate : ''
  const priorityId = typeof route.query.priority_id === 'string' ? route.query.priority_id : ''
  const reverse = typeof route.query.reverse === 'string' ? route.query.reverse : ''
  const homeKey = rerenderStore.homeKey.toString()
  return `${currentPage.value}-${search}-${locate}-${priorityId}-${reverse}-${homeKey}`
})

import { useDropZone } from '@vueuse/core'
import { useUploadStore } from '@/store/uploadStore'
function onDrop(files: File[] | null) {
  console.log(files)
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
const dropZoneRef = ref<HTMLDivElement>()

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

watchEffect(() => {
  console.log('isOverDropZone is', isOverDropZone.value)
})

onMounted(() => {
  window.addEventListener('dragenter', () => {
    console.log('A')

    visible.value = true
  })
})
</script>

<style scoped></style>
