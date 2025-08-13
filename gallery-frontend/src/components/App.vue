<template>
  <v-app
    @dragstart.prevent
    @dragover.prevent
    @drop.prevent
    @selectstart.prevent
    :style="{
      userSelect:
        scrollbarStore.isDragging || scrollbarStoreInsideAlbum.isDragging ? 'none' : 'auto' // Prevent accidental selection while scrolling.
    }"
  >
    <v-main class="h-screen">
      <DropZoneModal v-if="!configStore.isMobile" />
      <router-view v-slot="{ Component }" :key="routeKey">
        <component :is="Component" />
      </router-view> </v-main
    ><v-snackbar-queue v-model="messageStore.queue" timeout="2500" />
    <EditTagsModal v-if="modalStore.showEditTagsModal" />
    <EditAlbumsModal v-if="modalStore.showEditAlbumsModal" />
    <EditBatchTagsModal v-if="modalStore.showBatchEditTagsModal" />
    <EditBatchAlbumsModal v-if="modalStore.showBatchEditAlbumsModal" />
    <UploadModal v-if="modalStore.showUploadModal" />
    <SettingModal v-if="modalStore.showSettingModal" />
  </v-app>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import { computed, onBeforeMount } from 'vue'
import { useScrollbarStore } from '@/store/scrollbarStore'
import { useRerenderStore } from '@/store/rerenderStore'
import { useMessageStore } from '@/store/messageStore'
import DropZoneModal from './Modal/DropZoneModal.vue'
import { useConstStore } from '@/store/constStore'
import isMobile from 'is-mobile'
import { useConfigStore } from '@/store/configStore'
import EditTagsModal from '@/components/Modal/EditTagsModal.vue'
import EditBatchTagsModal from '@/components/Modal/EditBatchTagsModal.vue'
import UploadModal from '@/components/Modal/UploadModal.vue'
import EditAlbumsModal from '@/components/Modal/EditAlbumsModal.vue'
import EditBatchAlbumsModal from '@/components/Modal/EditBatchAlbumsModal.vue'
import SettingModal from '@/components/Modal/SettingModal.vue'
import { useModalStore } from '@/store/modalStore'

const modalStore = useModalStore('mainId')
const scrollbarStore = useScrollbarStore('mainId')
const scrollbarStoreInsideAlbum = useScrollbarStore('subId')
const rerenderStore = useRerenderStore('mainId')
const messageStore = useMessageStore('mainId')
const constStore = useConstStore('mainId')
const configStore = useConfigStore('mainId')
const route = useRoute()

// The routeKey is used to ensure that the router-view reloads the Home.vue component properly.
// Without it, Vue may cache the component for optimization, potentially causing bugs.
const routeKey = computed(() => {
  const currentPage = route.meta.baseName
  const search = typeof route.query.search === 'string' ? route.query.search : ''
  const locate = typeof route.query.locate === 'string' ? route.query.locate : ''
  const priorityId = typeof route.query.priority_id === 'string' ? route.query.priority_id : ''
  const reverse = typeof route.query.reverse === 'string' ? route.query.reverse : ''
  const concurrencyNumber = constStore.concurrencyNumber
  const homeKey = rerenderStore.homeKey.toString()
  return `${currentPage}-${search}-${locate}-${priorityId}-${reverse}-${concurrencyNumber}-${homeKey}`
})

onBeforeMount(async () => {
  // Load the subRowHeightScale / showInfo / concurrencyNumber from constStore when the app is mounted.
  await constStore.loadSubRowHeightScale()
  await constStore.loadShowInfo()
  await constStore.loadConcurrencyNumber() // ← 新增
  configStore.isMobile = isMobile()
})
</script>

<style>
/* Disable native dragging on common elements across the app */
img,
a,
svg,
video,
canvas {
  -webkit-user-drag: none;
}

/* Disable text selection across the app */
.v-application,
.v-application * {
  user-select: none;
  -webkit-user-select: none; /* Safari */
  -moz-user-select: none; /* Firefox */
  -webkit-touch-callout: none; /* iOS long-press menu */
}

/* Explicitly ensure images and common wrappers are not selectable */
img,
video {
  user-select: none !important;
  -webkit-user-select: none !important;
  -moz-user-select: none !important;
}

/* Allow selection where it makes sense */
input,
textarea,
[contenteditable='true'] {
  user-select: text;
  -webkit-user-select: text;
}
</style>
