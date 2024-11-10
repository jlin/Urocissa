<template>
  <v-col
    id="metadata-col"
    v-if="metadata"
    class="h-100 metadata-css"
    cols="auto"
    :style="{ backgroundColor: 'white' }"
  >
    <v-row no-gutters class="position-relative">
      <v-toolbar color="white">
        <!-- Icon button with increased size -->
        <v-btn icon @click="toggleInfo">
          <v-icon>mdi-close</v-icon>
        </v-btn>
        <!-- Wrapped Info text with increased font size -->
        <v-toolbar-title class="text-h5">Info</v-toolbar-title>
      </v-toolbar>
      <v-col class="h-100 w-100" cols="auto">
        <v-list bg-color="white" class="pa-0" height="100%" lines="two">
          <!-- Metadata Items -->
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
            <v-list-item-title class="text-wrap">{{ dater(metadata.timestamp) }}</v-list-item-title>
            <v-list-item-subtitle class="text-wrap">{{
              timer(metadata.timestamp)
            }}</v-list-item-subtitle>
          </v-list-item>
          <v-list-item
            v-if="metadata.exif_vec.Make !== undefined || metadata.exif_vec.Model !== undefined"
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
                <v-col cols="auto">{{ formatExifData(metadata.exif_vec).ExposureTime }}</v-col>
                <v-col cols="auto">{{ formatExifData(metadata.exif_vec).FocalLength }}</v-col>
                <v-col cols="auto">{{
                  formatExifData(metadata.exif_vec).PhotographicSensitivity
                }}</v-col>
              </v-row>
            </v-list-item-subtitle>
          </v-list-item>

          <!-- Tags Section -->
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
                v-for="tag in filteredTags"
                :key="tag"
                link
                class="ma-1"
                @click="searchByTag(tag)"
              >
                {{ tag }}
              </v-chip>
            </v-list-item-subtitle>
            <v-list-item-subtitle>
              <v-chip
                prepend-icon="mdi-pencil"
                color="black"
                variant="outlined"
                class="ma-1"
                link
                @click="openEditTagsModal"
                >edit</v-chip
              >
            </v-list-item-subtitle>
          </v-list-item>

          <!-- Albums Section -->
          <v-list-item>
            <template v-slot:prepend>
              <v-avatar>
                <v-icon color="black">mdi-image-album</v-icon>
              </v-avatar>
            </template>
            <v-list-item-subtitle class="text-wrap">
              <v-chip
                variant="flat"
                color="black"
                v-for="albumId in metadata.album"
                :key="albumId"
                link
                class="ma-1"
                @click="navigateToAlbum(albumId)"
              >
                {{ albumStore.albumMap.get(albumId)! }}
              </v-chip>
            </v-list-item-subtitle>
            <v-list-item-subtitle>
              <v-chip
                prepend-icon="mdi-pencil"
                color="black"
                variant="outlined"
                class="ma-1"
                link
                @click="openEditAlbumsModal"
                >edit</v-chip
              >
            </v-list-item-subtitle>
          </v-list-item>
        </v-list>
      </v-col>
    </v-row>
  </v-col>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useInfoStore } from '@/store/infoStore'
import { useModalStore } from '@/store/modalStore'
import { useAlbumStore } from '@/store/albumStore'
import { editTagsInWorker } from '@/script/inWorker/editTagsInWorker'
import { filesize } from 'filesize'
import { useDataStore } from '@/store/dataStore'
import { DataBase } from '@/script/common/types'

const props = defineProps<{
  metadata: DataBase
}>()

// Stores
const infoStore = useInfoStore()
const modalStore = useModalStore()
const albumStore = useAlbumStore()
const dataStore = useDataStore() // Import dataStore
const router = useRouter()

const filePathComplete = computed(() => {
  return `${props.metadata?.alias[0].file}`
})
const filePath = computed(() => {
  return `${filePathComplete.value.split('/').pop()}`
})
const separator = computed(() => {
  return filePath.value.includes('\\') ? '\\' : '/'
})

// Computed Properties
const filteredTags = computed(() =>
  props.metadata.tag.filter(
    (tag) => tag !== '_favorite' && tag !== '_archived' && tag !== '_trashed'
  )
)

// Retrieve index based on metadata hash
const index = computed(() => {
  return dataStore.hashMapData.get(props.metadata.hash)!
})

// Methods
function toggleInfo() {
  infoStore.showInfo = !infoStore.showInfo
}

function quickAddTags(tag: string) {
  const indexArray = [index.value]
  const addTagsArray: string[] = [tag]
  const removeTagsArray: string[] = []
  editTagsInWorker(indexArray, addTagsArray, removeTagsArray)
}

function quickRemoveTags(tag: string) {
  const indexArray = [index.value]
  const addTagsArray: string[] = []
  const removeTagsArray: string[] = [tag]
  editTagsInWorker(indexArray, addTagsArray, removeTagsArray)
}

async function searchByTag(tag: string) {
  await router.push({ path: '/all', query: { search: `tag: ${tag.trim()}` } })
}

function openEditTagsModal() {
  modalStore.showEditTagsModal = true
}

function navigateToAlbum(albumId: string) {
  const albumPath = `/album/${albumId}` // Adjust the path as necessary
  router.push({ path: albumPath })
}

function openEditAlbumsModal() {
  modalStore.showEditAlbumsModal = true
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

interface ExifData {
  FNumber: string // Aperture value as a string, e.g., "f/2.8"
  ExposureTime: string // Exposure time as a string, e.g., "1/60 s"
  FocalLength: string // Focal length as a string, e.g., "35 mm"
  PhotographicSensitivity: string
}

function formatExifData(exifData: any): ExifData {
  const formattedExifData: ExifData = {
    FNumber: exifData.FNumber.replace('f/', 'ƒ/'),
    ExposureTime: `1/${exifData.ExposureTime.replace(' s', '').replace('1/', '')}`,
    FocalLength: `${exifData.FocalLength.replace(' mm', '')} mm`,
    PhotographicSensitivity: `ISO ${exifData.PhotographicSensitivity}`
  }

  return formattedExifData
}

function dater(timestamp: number): string {
  const locale = navigator.language
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }).format(timestamp)
}

function timer(timestamp: number): string {
  const locale = navigator.language
  return new Intl.DateTimeFormat(locale, {
    weekday: 'long',
    hour: 'numeric',
    minute: 'numeric',
    second: 'numeric',
    hour12: true,
    dayPeriod: 'narrow',
    timeZoneName: 'short'
  }).format(timestamp)
}
</script>
<style scoped>
.metadata-css {
  width: 360px;
}

@media (max-width: 720px) {
  .metadata-css {
    width: 100%;
  }
}
</style>
