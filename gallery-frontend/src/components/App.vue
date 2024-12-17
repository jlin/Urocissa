<template>
  <v-app
    :style="{
      userSelect:
        scrollbarStore.isDragging || scrollbarStoreInsideAlbum.isDragging ? 'none' : 'auto' // Prevent accidental selection while scrolling.
    }"
  >
    <component :is="NavBar" v-if="route.name !== 'LoginPage'" />
    <v-main class="h-screen">
      <router-view v-slot="{ Component }" :key="routeKey">
        <component :is="Component" />
      </router-view>
    </v-main>
    <NotificationWarn />
  </v-app>
  <input
    v-if="!route.fullPath.includes('share')"
    id="upload-input"
    type="file"
    @change="uploadStore.handleFileUpload"
    ref="fileInput"
    multiple
    style="display: none"
  />
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed, Ref, watchEffect, ref } from 'vue'
import { useScrollbarStore } from '@/store/scrollbarStore'
import NotificationWarn from '@/components/NotificationWarn.vue'
import NavBar from '@/components/NavBar/NavBar.vue'
import { useUploadStore } from '@/store/uploadStore'

const uploadStore = useUploadStore('mainId')
const scrollbarStore = useScrollbarStore('mainId')
const scrollbarStoreInsideAlbum = useScrollbarStore('subId')

const route = useRoute()

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

const fileInput: Ref<HTMLInputElement | null> = ref(null)

watchEffect(() => {
  uploadStore.uploadButton = fileInput.value
})

// The routeKey is used to ensure that the router-view reloads the Home.vue component properly.
// Without it, Vue may cache the component for optimization, potentially causing bugs.
const routeKey = computed(() => {
  const search = typeof route.query.search === 'string' ? route.query.search : ''
  const locate = typeof route.query.locate === 'string' ? route.query.locate : ''
  const priorityId = typeof route.query.priority_id === 'string' ? route.query.priority_id : ''
  const reverse = typeof route.query.reverse === 'string' ? route.query.reverse : ''

  return `${currentPage.value}-${search}-${locate}-${priorityId}-${reverse}`
})
</script>

<style scoped></style>
