<template>
  <v-overlay
    :model-value="true"
    :height="'100%'"
    :width="'100%'"
    class="d-flex"
    id="view-page"
    persistent
  >
    <v-container
      fluid
      class="pa-0 h-100 overflow-hidden position-relative"
      :style="{
        backgroundColor: 'black'
      }"
    >
      <v-row no-gutters class="w-100 h-100 flex-nowrap">
        <v-col
          ref="colRef"
          cols="auto"
          :style="{
            transition: 'width 0.3s ease-in-out'
          }"
          :class="{ 'show-info': infoStore.showInfo, 'not-show-info': !infoStore.showInfo }"
          class="h-100"
        >
          <v-row no-gutters class="h-100 position-relative">
            <ViewPageToolBar :metadata="metadata" />
            <v-col v-if="metadata" id="col-ref" class="h-100 d-flex align-center justify-center">
              <v-img
                v-if="metadata?.ext_type === 'image'"
                :src="imgStore.imgOriginal.get(index)!"
                :lazy-src="imgStore.imgUrl.get(index)!"
                :style="{
                  width: `${metadata.width}px`,
                  height: `${metadata.height}px`,
                  maxWidth: '100%',
                  maxHeight: '100%'
                }"
                inline
              ></v-img>
              <video
                controls
                autoplay
                v-if="metadata?.ext_type === 'video'"
                :src="getSrc(hash, false, 'mp4', Cookies.get('jwt')!, undefined)"
                :style="{
                  width: `${metadata.width}px`,
                  height: `${metadata.height}px`,
                  maxWidth: '100%',
                  maxHeight: '100%'
                }"
                inline
              ></video>
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
        <v-col
          v-if="metadata"
          class="h-100 metadata-css"
          cols="auto"
          :style="{
            backgroundColor: 'white'
          }"
        >
          <v-row no-gutters class="position-relative">
            <v-toolbar color="white">
              <!-- Icon button with increased size -->
              <v-btn icon @click="infoStore.showInfo = !infoStore.showInfo">
                <v-icon>mdi-close</v-icon>
              </v-btn>
              <!-- Wrapped Info text with increased font size -->
              <v-toolbar-title class="text-h5">Info</v-toolbar-title>
            </v-toolbar>
            <v-col class="h-100 w-100" cols="auto">
              <v-list bg-color="white" class="pa-0" height="100%" lines="two">
                <v-list-item>
                  <template v-slot:prepend>
                    <v-avatar>
                      <v-icon color="black">mdi-image</v-icon>
                    </v-avatar>
                  </template>
                  <v-list-item-title class="text-wrap">{{
                    `${metadata.width} \u00D7 ${metadata.height}`
                  }}</v-list-item-title>
                  <v-list-item-subtitle class="text-wrap">{{
                    filesize(metadata.size)
                  }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <template v-slot:prepend>
                    <v-avatar>
                      <v-icon color="black">mdi-folder</v-icon>
                    </v-avatar>
                  </template>
                  <v-list-item-title class="text-wrap">{{
                    `${filePath.split(separator).pop() || ''}`
                  }}</v-list-item-title>
                  <v-list-item-subtitle class="text-wrap">{{
                    `${filePathComplete}`
                  }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <template v-slot:prepend>
                    <v-avatar>
                      <v-icon color="black">mdi-calendar</v-icon>
                    </v-avatar>
                  </template>
                  <v-list-item-title class="text-wrap">{{
                    dater(metadata.timestamp)
                  }}</v-list-item-title>
                  <v-list-item-subtitle class="text-wrap">{{
                    timer(metadata.timestamp)
                  }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item
                  v-if="
                    metadata.exif_vec.Make !== undefined || metadata.exif_vec.Model !== undefined
                  "
                >
                  <template v-slot:prepend>
                    <v-avatar>
                      <v-icon color="black">mdi-camera-iris</v-icon>
                    </v-avatar>
                  </template>
                  <v-list-item-title class="text-wrap">{{
                    generateExifMake(metadata.exif_vec)
                  }}</v-list-item-title>
                  <v-list-item-subtitle class="text-wrap">
                    <v-row>
                      <v-col cols="auto">{{ formatExifData(metadata.exif_vec).FNumber }}</v-col>
                      <v-col cols="auto">{{
                        formatExifData(metadata.exif_vec).ExposureTime
                      }}</v-col>
                      <v-col cols="auto">{{ formatExifData(metadata.exif_vec).FocalLength }}</v-col>
                      <v-col cols="auto">{{
                        formatExifData(metadata.exif_vec).PhotographicSensitivity
                      }}</v-col>
                    </v-row>
                  </v-list-item-subtitle>
                </v-list-item>
                <v-divider></v-divider>
                <v-list-item>
                  <template v-slot:prepend>
                    <v-avatar>
                      <v-icon color="black">mdi-tag</v-icon>
                    </v-avatar>
                  </template>
                  <v-list-item-title>
                    <v-chip
                      v-if="metadata.tag.includes('_favorite')"
                      prepend-icon="mdi-star"
                      color="black"
                      variant="tonal"
                      class="ma-1"
                      link
                      @click="quickRemoveTags('_favorite')"
                      >favorite</v-chip
                    >
                    <v-chip
                      v-else
                      prepend-icon="mdi-star-outline"
                      color="grey"
                      variant="tonal"
                      class="ma-1"
                      link
                      @click="quickAddTags('_favorite')"
                      >favorite</v-chip
                    >
                    <v-chip
                      v-if="metadata.tag.includes('_archived')"
                      prepend-icon="mdi-archive-arrow-down"
                      color="black"
                      variant="tonal"
                      class="ma-1"
                      link
                      @click="quickRemoveTags('_archived')"
                      >archived</v-chip
                    >
                    <v-chip
                      v-else
                      prepend-icon="mdi-archive-arrow-down"
                      color="grey"
                      variant="tonal"
                      class="ma-1"
                      link
                      @click="quickAddTags('_archived')"
                      >archived</v-chip
                    >
                  </v-list-item-title>
                  <v-list-item-subtitle class="text-wrap">
                    <v-chip
                      variant="flat"
                      color="black"
                      v-for="tag in metadata.tag.filter((tag) => {
                        return tag !== '_favorite' && tag !== '_archived' && tag !== '_trashed'
                      })"
                      :key="tag"
                      link
                      class="ma-1"
                      @click="searchByTag(tag)"
                    >
                      {{ tag }}</v-chip
                    >
                  </v-list-item-subtitle>
                  <v-list-item-subtitle>
                    <v-chip
                      prepend-icon="mdi-pencil"
                      color="black"
                      variant="outlined"
                      class="ma-1"
                      link
                      @click="modalStore.showEditTagsModal = true"
                      >edit</v-chip
                    >
                  </v-list-item-subtitle>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
        </v-col>
      </v-row>
    </v-container>
  </v-overlay>
</template>

<script setup lang="ts">
import { ref, watchEffect, onMounted, onUnmounted, computed, onBeforeUnmount } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDataStore } from '@/store/dataStore'
import { VCol } from 'vuetify/components'
import ViewPageToolBar from '@/components/Home/View/ViewPageToolBar.vue'
import { useInfoStore } from '@/store/infoStore'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { useModalStore } from '@/store/modalStore'
import { useInitializedStore } from '@/store/initializedStore'
import { filesize } from 'filesize'
import { useImgStore } from '@/store/imgStore'
import { bindActionDispatch } from 'typesafe-agent-events'
import { toImgWorker } from '@/worker/workerApi'
import { useWorkerStore } from '@/store/workerStore'
import { useQueueStore } from '@/store/queueStore'
import { batchNumber } from '@/script/common/commonType'
import Cookies from 'js-cookie'
import { fetchDataInWorker } from '@/script/inWorker/fetchDataInWorker'
import { useDataLengthStore } from '@/store/dataLengthStore'
import { getSrc } from '@/../config.ts'

interface ExifData {
  FNumber: string // Aperture value as a string, e.g., "f/2.8"
  ExposureTime: string // Exposure time as a string, e.g., "1/60 s"
  FocalLength: string // Focal length as a string, e.g., "35 mm"
  PhotographicSensitivity: string
}
const dataLengthStore = useDataLengthStore()
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
const hashPattern = /^[a-fA-F0-9]{64}$/
const metadata = computed(() => {
  return dataStore.data.get(index.value)
})
const nextHash = computed(() => {
  return dataStore.data.get(index.value + 1)?.hash
})
const previousHash = computed(() => {
  return dataStore.data.get(index.value - 1)?.hash
})
const errorMessage = ref<string | null>(null)
const colRef = ref<InstanceType<typeof VCol> | null>(null)
const filePathComplete = computed(() => {
  return `${metadata.value?.alias[0].file}`
})
const filePath = computed(() => {
  return `${filePathComplete.value.split('/').pop()}`
})
const separator = computed(() => {
  return filePath.value.includes('\\') ? '\\' : '/'
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
    postToWorker.processImage({
      index: index,
      hash: dataStore.data.get(index)!.hash,
      devicePixelRatio: window.devicePixelRatio,
      jwt: Cookies.get('jwt')!
    })
    return false
  } else {
    return false
  }
}

function quickAddTags(tag: string) {
  if (metadata.value !== undefined) {
    const indexArray = [index.value]
    let addTagsArray: string[] = [tag]
    let removeTagsArray: string[] = []
    editTagsInWorker(indexArray, addTagsArray, removeTagsArray)
  }
}

function quickRemoveTags(tag: string) {
  if (metadata.value !== undefined) {
    const indexArray = [index.value]

    let addTagsArray: string[] = []
    let removeTagsArray: string[] = [tag]
    editTagsInWorker(indexArray, addTagsArray, removeTagsArray)
  }
}

async function searchByTag(tag: string) {
  await router.push({ path: '/all', query: { search: `tag: ${tag.trim()}` } })
}

function dater(timestamp: number): string {
  const locale = navigator.language // Gets the user's preferred language setting from the browser
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }).format(timestamp)
}

function timer(timestamp: number): string {
  const locale = navigator.language // Gets the user's preferred language setting from the browser
  return new Intl.DateTimeFormat(locale, {
    weekday: 'long',
    hour: 'numeric',
    minute: 'numeric',
    second: 'numeric',
    hour12: true, // Use 12-hour format
    dayPeriod: 'narrow', // For showing AM/PM or similar local equivalents
    timeZoneName: 'short'
  }).format(timestamp)
}

function formatExifData(exifData: any): ExifData {
  const formattedExifData: ExifData = {
    FNumber: exifData.FNumber.replace('f/', 'ƒ/'), // Formats aperture notation
    ExposureTime: `1/${exifData.ExposureTime.replace(' s', '').replace('1/', '')}`, // Formats exposure time
    FocalLength: `${exifData.FocalLength.replace(' mm', '')} mm`, // Formats focal length
    PhotographicSensitivity: `ISO ${exifData.PhotographicSensitivity}` // Formats ISO
  }

  return formattedExifData
}

function generateExifMake(exifData: any): string {
  let make_formated: string = ''
  let model_formated: string = ''
  if (exifData.Make !== undefined) {
    const make: string = exifData.Make.replace(/"/g, '')
    make_formated = make
      .split(',')
      .map((part) => part.trim())
      .filter((part) => part !== '')
      .join(', ')
  }
  if (exifData.Model !== undefined) {
    const model: string = exifData.Model.replace(/"/g, '')
    model_formated = model
      .split(',')
      .map((part) => part.trim())
      .filter((part) => part !== '')
      .join(', ')
  }

  return make_formated + ' ' + model_formated
}

function prefetchMedia(index: number) {
  const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

  const prefetch = async () => {
    for (let i = 1; i <= 10; i++) {
      const nextIndex = index + i
      const prevIndex = index - i

      const nextMeta = dataStore.data.get(nextIndex)
      const prevMeta = dataStore.data.get(prevIndex)

      if (nextMeta !== undefined && nextMeta.ext_type === 'image') {
        checkAndFetch(nextIndex)
      } else if (nextMeta === undefined && nextIndex <= dataLengthStore.dataLength - 1) {
        fetchDataInWorker(Math.floor(nextIndex / batchNumber))
      }

      if (prevMeta !== undefined && prevMeta.ext_type === 'image') {
        checkAndFetch(prevIndex)
      } else if (prevMeta === undefined && prevIndex >= 0) {
        fetchDataInWorker(Math.floor(prevIndex / batchNumber))
      }

      await delay(100) // Delay each prefetch by 100ms
    }
  }

  prefetch().catch((error) => console.error('Error prefetching media:', error))
}

watchEffect(async () => {
  if (initializedStore.initialized) {
    if (hashPattern.test(hash.value)) {
      if (index.value !== undefined) {
        checkAndFetch(index.value)
        if (metadata.value) {
          console.log(metadata.value)
          errorMessage.value = null

          // Prefetch next and previous 10 hashes if they exist
          prefetchMedia(index.value)
        }
      }
    }
  }
})

onMounted(() => {
  watchEffect(async () => {
    if (initializedStore.initialized) {
      if (hashPattern.test(hash.value)) {
        if (index.value !== undefined) {
          checkAndFetch(index.value)
          if (metadata.value) {
            console.log(metadata.value)
            errorMessage.value = null

            // Prefetch next and previous 10 hashes if they exist
            prefetchMedia(index.value)
          }
        }
      }
    }
  })
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
.v-container::-webkit-scrollbar {
  display: none;
  /* Hide scrollbar */
}

.my-toolbar {
  z-index: 1;
  background: linear-gradient(
    to bottom,
    rgba(0, 0, 0, 0.5) 0%,
    rgba(0, 0, 0, 0.25) 50%,
    rgba(0, 0, 0, 0) 100%
  );
}

.wrap {
  white-space: normal;
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

.metadata-css {
  width: 360px;
}

@media (max-width: 720px) {
  .metadata-css {
    width: 100%;
  }
}
</style>
