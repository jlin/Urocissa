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
              `${metadata.width} × ${metadata.height}`
            }}</v-list-item-title>
            <v-list-item-subtitle class="text-wrap">{{
              filesize(metadata.size)
            }}</v-list-item-subtitle>
          </v-list-item>

          <!-- Additional Metadata Items... (Repeat similar structure) -->

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
</script>
