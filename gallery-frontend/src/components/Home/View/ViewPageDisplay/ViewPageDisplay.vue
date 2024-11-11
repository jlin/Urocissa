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
      <ViewPageDisplayDatabase
        :index="index"
        :metadata="metadata"
        :colWidth="colWidth"
        :colHeight="colHeight"
      />
      <ViewPageDisplayAlbum
        :index="index"
        :metadata="metadata"
        :colWidth="colWidth"
        :colHeight="colHeight"
      />
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
      <v-col v-if="!metadata" class="h-100 d-flex align-center justify-center">
        <v-progress-circular color="primary" indeterminate></v-progress-circular>
      </v-col>
    </v-row>
  </v-col>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, onBeforeUnmount, watch } from 'vue'
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
import { AbstractData } from '@/script/common/types'
import { useElementSize } from '@vueuse/core'
import ViewPageDisplayDatabase from '@/components/Home/View/ViewPageDisplay/ViewPageDisplayDatabase.vue'
import ViewPageDisplayAlbum from '@/components/Home/View/ViewPageDisplay/ViewPageDisplayAlbum.vue'

const colRef = ref<InstanceType<typeof VCol> | null>(null)
const { width: colWidth, height: colHeight } = useElementSize(colRef)

const props = defineProps<{
  metadata: AbstractData
  index: number
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

const nextHash = computed(() => {
  const nextData = dataStore.data.get(props.index + 1)
  if (nextData !== undefined && nextData.database) {
    return nextData.database.hash
  } else if (nextData !== undefined && nextData.album) {
    return nextData.album.id
  } else {
    return undefined
  }
})

const previousHash = computed(() => {
  const previousData = dataStore.data.get(props.index - 1)
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
  return props.index % workerStore.concurrencyNumber
})

const postToWorker = bindActionDispatch(toImgWorker, (action) => {
  workerStore.imgWorker[workerIndex.value]!.postMessage(action)
})

const checkAndFetch = (index: number): boolean => {
  if (imgStore.imgOriginal.has(index)) {
    return true
  } else if (!queueStore.original.has(index)) {
    queueStore.original.add(index)
    const hash = dataStore.data.get(index)!.database
      ? dataStore.data.get(index)!.database!.hash
      : dataStore.data.get(index)!.album!.cover
    if (hash !== null) {
      postToWorker.processImage({
        index: index,
        hash: hash,
        devicePixelRatio: window.devicePixelRatio,
        jwt: Cookies.get('jwt')!
      })
    }
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
      if (props.index !== undefined) {
        checkAndFetch(props.index)

        // Prefetch next and previous 10 hashes if they exist
        prefetchMedia(props.index)
        // console.log(props.metadata) // debug usage
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
