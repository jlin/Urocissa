<template>
  <v-col
    id="image-display-col"
    ref="colRef"
    cols="auto"
    :style="{ transition: 'width 0.3s ease-in-out' }"
    :class="{ 'show-info': infoStore.showInfo, 'not-show-info': !infoStore.showInfo }"
    class="h-100"
  >
    <v-row no-gutters class="h-100 position-relative">
      <ViewPageToolBar :metadata="metadata" />
      <v-col v-if="metadata" id="col-ref" class="h-100 d-flex align-center justify-center">
        <v-img
          v-if="metadata && metadata.database && metadata.database.ext_type === 'image'"
          :src="imgStore.imgOriginal.get(index)!"
          :lazy-src="imgStore.imgUrl.get(index)!"
          :style="{
            width: `${metadata.database.width}px`,
            height: `${metadata.database.height}px`,
            maxWidth: '100%',
            maxHeight: '100%'
          }"
          inline
        ></v-img>
        <video
          controls
          autoplay
          v-if="
            metadata &&
            metadata.database &&
            metadata.database.ext_type === 'video' &&
            !metadata.database.pending
          "
          :src="getSrc(hash, false, 'mp4', Cookies.get('jwt')!, undefined)"
          :style="{
            width: `${metadata.database.width}px`,
            height: `${metadata.database.height}px`,
            maxWidth: '100%',
            maxHeight: '100%'
          }"
          inline
        ></video>
        <v-img
          rounded="xl"
          aspect-ratio="1"
          cover
          v-if="metadata && metadata.album"
          :src="imgStore.imgOriginal.get(index)!"
          :lazy-src="imgStore.imgUrl.get(index)!"
          :style="{
            width: `${Math.round(
              metadata.album.width *
                Math.min(colWidth / metadata.album.width, colHeight / metadata.album.height) *
                0.85
            )}px`,
            height: `${Math.round(
              metadata.album.height *
                Math.min(colWidth / metadata.album.width, colHeight / metadata.album.height) *
                0.85
            )}px`,
            border: '8px solid white'
          }"
          inline
        >
          <v-chip
            id="album-label-chip"
            density="default"
            size="x-large"
            prepend-icon="mdi-image-album"
            color="black"
            variant="flat"
            class="position-absolute ma-2"
            :style="{
              bottom: '10px',
              right: '10px',
              zIndex: 4
            }"
          >
            <span class="text-truncate">
              {{ metadata.album.title }}
            </span>
          </v-chip>
        </v-img>

        <v-card
          v-if="metadata?.database?.pending"
          class="d-flex align-center justify-start"
          outlined
          style="padding: 16px"
        >
          <v-row align="center" no-gutters>
            <v-col cols="auto" class="d-flex align-center">
              <v-icon size="48" color="warning">mdi-alert-circle-outline</v-icon>
            </v-col>
            <v-col class="text-left pl-4">
              <div>This video is currently being processed.</div>
              <div>Please check back later.</div>
            </v-col>
          </v-row>
        </v-card>

        <v-card
          id="previous-page-anchor"
          v-if="previousHash !== undefined"
          color="transparent"
          class="navigate-left h-100 d-flex align-center justify-center"
          style="position: absolute; left: 0"
          :to="{ path: previousPage, query: $route.query }"
        >
          <v-icon>mdi-arrow-left</v-icon>
        </v-card>
        <v-card
          id="next-page-anchor"
          v-if="nextHash !== undefined"
          color="transparent"
          class="navigate-right h-100 d-flex align-center justify-center"
          style="position: absolute; right: 0"
          :to="{ path: nextPage, query: $route.query }"
        >
          <v-icon>mdi-arrow-right</v-icon>
        </v-card>
      </v-col>
      <v-col v-else class="h-100 d-flex align-center justify-center">
        <v-progress-circular color="primary" indeterminate></v-progress-circular>
      </v-col>
    </v-row>
  </v-col>
</template>

<script setup lang="ts">
import { ref, watchEffect, onMounted, onUnmounted, computed, onBeforeUnmount, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import { VCol } from 'vuetify/components'
import ViewPageToolBar from '@/components/Home/View/ViewPageToolBar.vue'
import { useInfoStore } from '@/store/infoStore'
import { useModalStore } from '@/store/modalStore'
import { useInitializedStore } from '@/store/initializedStore'
import { useImgStore } from '@/store/imgStore'
import { bindActionDispatch } from 'typesafe-agent-events'
import { toImgWorker } from '@/worker/workerApi'
import { useWorkerStore } from '@/store/workerStore'
import { useQueueStore } from '@/store/queueStore'
import { batchNumber } from '@/script/common/constants'
import Cookies from 'js-cookie'
import { fetchDataInWorker } from '@/script/inWorker/fetchDataInWorker'
import { usePrefetchStore } from '@/store/prefetchStore'
import { getSrc } from '@/../config.ts'
import { AbstractData } from '@/script/common/types'
import { useElementSize } from '@vueuse/core'

const colRef = ref<InstanceType<typeof VCol> | null>(null)
const { width: colWidth, height: colHeight } = useElementSize(colRef)

const props = defineProps<{
  metadata: AbstractData
}>()

const prefetchStore = usePrefetchStore()
const workerStore = useWorkerStore()
const queueStore = useQueueStore()
const imgStore = useImgStore()
const initializedStore = useInitializedStore()
const modalStore = useModalStore()
const infoStore = useInfoStore()
const dataStore = useDataStore()
const route = useRoute()
const router = useRouter()

const hash = computed(() => {
  return route.params.hash as string
})

const nextHash = computed(() => {
  const nextData = dataStore.data.get(index.value + 1)
  if (nextData !== undefined && nextData.database) {
    return nextData.database.hash
  } else if (nextData !== undefined && nextData.album) {
    return nextData.album.id
  } else {
    return undefined
  }
})

const previousHash = computed(() => {
  const previousData = dataStore.data.get(index.value - 1)
  if (previousData !== undefined && previousData.database) {
    return previousData.database.hash
  } else if (previousData !== undefined && previousData.album) {
    return previousData.album.id
  } else {
    return undefined
  }
})

const index = computed(() => {
  return dataStore.hashMapData.get(route.params.hash as string)!
})

const previousPage = computed(() => {
  if (route.path.startsWith('/favorite')) {
    return `/favorite/view/${previousHash.value}`
  } else if (route.path.startsWith('/archived')) {
    return `/archived/view/${previousHash.value}`
  } else if (route.path.startsWith('/all')) {
    return `/all/view/${previousHash.value}`
  } else if (route.path.startsWith('/trashed')) {
    return `/trashed/view/${previousHash.value}`
  } else if (route.path.startsWith('/album-')) {
    // Extract the album identifier
    const albumId = route.path.split('/')[1]
    return `/${albumId}/view/${previousHash.value}`
  } else {
    return `/view/${previousHash.value}`
  }
})

const nextPage = computed(() => {
  if (route.path.startsWith('/favorite')) {
    return `/favorite/view/${nextHash.value}`
  } else if (route.path.startsWith('/archived')) {
    return `/archived/view/${nextHash.value}`
  } else if (route.path.startsWith('/all')) {
    return `/all/view/${nextHash.value}`
  } else if (route.path.startsWith('/trashed')) {
    return `/trashed/view/${nextHash.value}`
  } else if (route.path.startsWith('/album-')) {
    // Extract the album identifier
    const albumId = route.path.split('/')[1]
    return `/${albumId}/view/${nextHash.value}`
  } else {
    return `/view/${nextHash.value}`
  }
})

const workerIndex = computed(() => {
  return index.value % workerStore.concurrencyNumber
})

const postToWorker = bindActionDispatch(toImgWorker, (action) =>
  workerStore.imgWorker[workerIndex.value]!.postMessage(action)
)

const checkAndFetch = (index: number): boolean => {
  if (imgStore.imgOriginal.has(index)) {
    return true
  } else if (!queueStore.original.has(index)) {
    queueStore.original.add(index)
    const hash = dataStore.data.get(index)!.database
      ? dataStore.data.get(index)!.database!.hash
      : dataStore.data.get(index)!.album!.cover!
    postToWorker.processImage({
      index: index,
      hash: hash,
      devicePixelRatio: window.devicePixelRatio,
      jwt: Cookies.get('jwt')!
    })
    return false
  } else {
    return false
  }
}

function prefetchMedia(index: number) {
  const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

  const prefetch = async () => {
    for (let i = 1; i <= 10; i++) {
      const nextIndex = index + i
      const prevIndex = index - i

      const nextMeta = dataStore.data.get(nextIndex)?.database!
      const prevMeta = dataStore.data.get(prevIndex)?.database!

      if (nextMeta !== undefined && nextMeta.ext_type === 'image') {
        checkAndFetch(nextIndex)
      } else if (nextMeta === undefined && nextIndex <= prefetchStore.dataLength - 1) {
        fetchDataInWorker(Math.floor(nextIndex / batchNumber))
      }

      if (prevMeta !== undefined && prevMeta.ext_type === 'image') {
        checkAndFetch(prevIndex)
      } else if (prevMeta === undefined && prevIndex >= 0) {
        fetchDataInWorker(Math.floor(prevIndex / batchNumber))
      }

      await delay(100)
    }
  }

  prefetch().catch((error) => console.error('Error prefetching media:', error))
}

watch(
  [index, () => initializedStore.initialized],
  () => {
    if (initializedStore.initialized) {
      if (index.value !== undefined) {
        checkAndFetch(index.value)

        // Prefetch next and previous 10 hashes if they exist
        prefetchMedia(index.value)
        console.log(props.metadata)
      }
    }
  },
  { immediate: true } // Executes the watcher immediately on component mount
)

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

function handleKeyDown(event: KeyboardEvent) {
  if (modalStore.showEditTagsModal) {
    return
  }
  let anchor: HTMLElement | null = null
  if (event.key === 'ArrowRight') {
    anchor = document.getElementById('next-page-anchor')
  } else if (event.key === 'ArrowLeft') {
    anchor = document.getElementById('previous-page-anchor')
  }
  if (anchor) {
    anchor.click()
  }
}

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
  } else if (path.startsWith('/album-') && path.includes('/view/')) {
    // Extract the album identifier
    const segments = path.split('/')
    const albumId = segments.find((segment) => segment.startsWith('album-'))
    return `/${albumId}`
  } else {
    return '/'
  }
})

watchEffect(() => {
  console.log('computedPath is', computedPath.value)
})

const handlePopState = () => {
  router.push({ path: computedPath.value, query: route.query })
}

window.addEventListener('popstate', handlePopState)

onBeforeUnmount(() => {
  window.removeEventListener('popstate', handlePopState)
})
</script>

<style scoped>
.my-toolbar {
  z-index: 1;
  background: linear-gradient(
    to bottom,
    rgba(0, 0, 0, 0.5) 0%,
    rgba(0, 0, 0, 0.25) 50%,
    rgba(0, 0, 0, 0) 100%
  );
}

.show-info {
  width: calc(100% - 360px);
}

@media (max-width: 720px) {
  .show-info {
    display: none;
  }
}

.not-show-info {
  width: 100%;
}
</style>
