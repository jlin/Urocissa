<template>
  <v-col
    id="metadata-col"
    v-if="metadata"
    class="h-100 metadata-css"
    cols="auto"
    :style="{ backgroundColor: 'white' }"
  >
    <v-row v-if="metadata.database" no-gutters class="position-relative">
      <v-toolbar
        color="white"
        :style="{
          backgroundColor: '#212121'
        }"
      >
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
          <ItemSize :database="metadata.database" />
          <ItemPath :database="metadata.database" />
          <v-list-item>
            <template #prepend>
              <v-avatar>
                <v-icon color="black">mdi-calendar</v-icon>
              </v-avatar>
            </template>
            <v-list-item-title class="text-wrap">{{
              dater(metadata.database.timestamp)
            }}</v-list-item-title>
            <v-list-item-subtitle class="text-wrap">{{
              timer(metadata.database.timestamp)
            }}</v-list-item-subtitle>
          </v-list-item>
          <ItemExif
            v-if="
              metadata.database.exif_vec.Make !== undefined ||
              metadata.database.exif_vec.Model !== undefined
            "
            :database="metadata.database"
          />
          <!-- Tags Section -->
          <v-divider></v-divider>
          <v-list-item>
            <template #prepend>
              <v-avatar>
                <v-icon color="black">mdi-tag</v-icon>
              </v-avatar>
            </template>
            <v-list-item-title>
              <v-chip
                v-if="metadata.database.tag.includes('_favorite')"
                prepend-icon="mdi-star"
                color="black"
                variant="tonal"
                class="ma-1"
                link
                @click="quickRemoveTags('_favorite', [index], isolationId)"
                >favorite</v-chip
              >
              <v-chip
                v-else
                prepend-icon="mdi-star-outline"
                color="grey"
                variant="tonal"
                class="ma-1"
                link
                @click="quickAddTags('_favorite', [index], isolationId)"
                >favorite</v-chip
              >
              <v-chip
                v-if="metadata.database.tag.includes('_archived')"
                prepend-icon="mdi-archive-arrow-down"
                color="black"
                variant="tonal"
                class="ma-1"
                link
                @click="quickRemoveTags('_archived', [index], isolationId)"
                >archived</v-chip
              >
              <v-chip
                v-else
                prepend-icon="mdi-archive-arrow-down"
                color="grey"
                variant="tonal"
                class="ma-1"
                link
                @click="quickAddTags('_archived', [index], isolationId)"
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
                @click="searchByTag(tag, router)"
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
            <template #prepend>
              <v-avatar>
                <v-icon color="black">mdi-image-album</v-icon>
              </v-avatar>
            </template>
            <v-list-item-subtitle class="text-wrap">
              <v-chip
                variant="flat"
                color="black"
                v-for="albumId in metadata.database.album"
                :key="albumId"
                link
                class="ma-1"
                @click="navigateToAlbum(albumId, router)"
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
    <v-row v-if="metadata.album" no-gutters class="position-relative">
      <v-toolbar
        color="white"
        :style="{
          backgroundColor: '#212121'
        }"
      >
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
            <template #prepend>
              <v-avatar>
                <v-icon color="black">mdi-image-album</v-icon>
              </v-avatar>
            </template>
            <v-list-item-title class="text-wrap">{{ metadata.album.title }}</v-list-item-title>
          </v-list-item>
          <v-list-item>
            <template #prepend>
              <v-avatar>
                <v-icon color="black">mdi-image</v-icon>
              </v-avatar>
            </template>
            <v-list-item-title class="text-wrap">{{
              `${metadata.album.itemCount} items`
            }}</v-list-item-title>
            <v-list-item-subtitle class="text-wrap">
              {{ filesize(metadata.album.itemSize) }}
            </v-list-item-subtitle>
          </v-list-item>
          <!-- Tags Section -->
          <v-divider></v-divider>
          <v-list-item>
            <template #prepend>
              <v-avatar>
                <v-icon color="black">mdi-tag</v-icon>
              </v-avatar>
            </template>
            <v-list-item-title>
              <v-chip
                v-if="metadata.album.tag.includes('_favorite')"
                prepend-icon="mdi-star"
                color="black"
                variant="tonal"
                class="ma-1"
                link
                @click="quickRemoveTags('_favorite', [index], isolationId)"
                >favorite</v-chip
              >
              <v-chip
                v-else
                prepend-icon="mdi-star-outline"
                color="grey"
                variant="tonal"
                class="ma-1"
                link
                @click="quickAddTags('_favorite', [index], isolationId)"
                >favorite</v-chip
              >
              <v-chip
                v-if="metadata.album.tag.includes('_archived')"
                prepend-icon="mdi-archive-arrow-down"
                color="black"
                variant="tonal"
                class="ma-1"
                link
                @click="quickRemoveTags('_archived', [index], isolationId)"
                >archived</v-chip
              >
              <v-chip
                v-else
                prepend-icon="mdi-archive-arrow-down"
                color="grey"
                variant="tonal"
                class="ma-1"
                link
                @click="quickAddTags('_archived', [index], isolationId)"
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
                @click="searchByTag(tag, router)"
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
        </v-list>
      </v-col>
    </v-row>
  </v-col>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useInfoStore } from '@/store/infoStore'
import { useModalStore } from '@/store/modalStore'
import { useAlbumStore } from '@/store/albumStore'
import { filesize } from 'filesize'
import { AbstractData, IsolationId } from '@/script/common/types'
import { dater, searchByTag } from '@/script/common/functions'
import { quickRemoveTags, quickAddTags } from '@/script/common/quickEditTags'
import { navigateToAlbum } from '@/script/navigator'
import ItemExif from './Item/ItemExif.vue'
import ItemSize from './Item/ItemSize.vue'
import ItemPath from './Item/ItemPath.vue'

const props = defineProps<{
  isolationId: IsolationId
  hash: string
  index: number
  metadata: AbstractData
}>()

// Stores

const infoStore = useInfoStore('mainId')

const modalStore = useModalStore('mainId')
const albumStore = useAlbumStore('mainId')
const router = useRouter()

// Computed Properties
const filteredTags = computed(() => {
  if (props.metadata.database) {
    return props.metadata.database.tag.filter(
      (tag) => tag !== '_favorite' && tag !== '_archived' && tag !== '_trashed'
    )
  } else if (props.metadata.album) {
    return props.metadata.album.tag.filter(
      (tag) => tag !== '_favorite' && tag !== '_archived' && tag !== '_trashed'
    )
  } else {
    // Throw an error if neither database nor album metadata is available
    throw new Error('Invalid metadata: Neither database nor album is available for filtering tags.')
  }
})

// Methods
function toggleInfo() {
  infoStore.showInfo = !infoStore.showInfo
}

function openEditTagsModal() {
  modalStore.showEditTagsModal = true
}

function openEditAlbumsModal() {
  modalStore.showEditAlbumsModal = true
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
watch(
  () => props.hash,
  () => {
    console.log(props.metadata)
  }
)
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
